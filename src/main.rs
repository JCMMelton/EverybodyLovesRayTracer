extern crate image;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;
use image::{ImageBuffer, Pixel, Rgb};
use std::f32;
use std::sync::{Mutex, Arc};

mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hit;
use hit::*;
mod sphere;
use sphere::*;
mod world;
use world::*;
mod camera;
use camera::*;
mod material;
use material::*;
mod utils;
mod bounding_box;
use bounding_box::*;
mod rectangle;
use rectangle::*;
//mod bvh;

fn color(ray: Ray, world: &World, depth: i32) -> Vec3 {
    let hit_record: HitRecord = HitRecord::new(
        Material::new_blank()
    );
    let world_hit: (bool, HitRecord) = world.hit(&ray, 0.0001, f32::MAX, &hit_record);
    if world_hit.0 {
        let scattered: Ray = Ray::new_empty();
        let attenuation: Vec3 = Vec3::from_value(0.0);
        let scatter_result: (bool, Ray, Vec3) = (world_hit.1).mat.scatter(&ray, &world_hit.1, &attenuation, &scattered);
        let emitted: Vec3 = (world_hit.1).mat.emitted();
        if depth < 50 && scatter_result.0 {
            return emitted + scatter_result.2 * color(scatter_result.1, &world, depth+1);
        }
        else {
            return emitted;
        }
    }
    else {
        return Vec3::from_value(0.0);
    }
}

fn main() {
    let nx: u32 = 1600;
    let ny: u32 = 1200;
    let ns: u32 = 400;
    let fx: f32 = nx as f32;
    let fy: f32 = ny as f32;
    let img = ImageBuffer::new(nx, ny);

    let lookfrom: Vec3 = Vec3::new( 0.0, 0.0,  1.0);
    let lookat:   Vec3 = Vec3::new( 0.0, 0.0, -1.0);
    let dist_to_focus: f32 = (lookfrom-lookat).length();
    let aperture: f32 = 0.1;

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        fx/fy,
        aperture,
        dist_to_focus
    );
    let world: World = World::single_sphere();
//    let world: World = World::test_scene();
    world.get_z_orders();
    println!("{:?}", world.bounding_box(0.0, 1.0));
    let depth: i32 = 0;
    let safe_img = Arc::new(Mutex::new(img));
    (0..ny).into_par_iter().for_each(|j| {
        (0..nx).into_par_iter().for_each(|i| {
            let mut col: Vec3 = Vec3::from_value(0.0);
            for _ in 0..ns {
                let u:  f32  = ((i as f32) + rand::random::<f32>())/fx;
                let v:  f32  = (((ny-j) as f32) + rand::random::<f32>())/fy;
                let r:  Ray  = cam.get_ray(u, v);
                let _p: Vec3 = r.point_at_parameter(2.0);
                col += color(r, &world, depth);
            }
            col /= fx;
            col = Vec3::new(f32::sqrt(col.r()), f32::sqrt(col.g()), f32::sqrt(col.b()) );
            let r: u8 = (255.99 * col.r()) as u8;
            let g: u8 = (255.99 * col.g()) as u8;
            let b: u8 = (255.99 * col.b()) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            safe_img.lock().unwrap().put_pixel(i, j, pixel);
        });
    });
    let _ = safe_img.lock().unwrap().save("assets/test.png");
}
