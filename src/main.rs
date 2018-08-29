extern crate image;
extern crate rand;

use image::{ImageBuffer, Pixel, Rgb};
use std::f32;
use rand::random;

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

fn color(ray: Ray, world: &World) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new();
    if world.hit(&ray, 0.0, 255.0, &mut hit_record) {
        return 0.5 * Vec3::new(hit_record.normal.x()+1.0, hit_record.normal.y()+1.0, hit_record.normal.z()+1.0);
    }
    else {
        let unit_direction: Vec3 = Vec3::unit_vector(ray.direction());
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns: u32 = 100;
    let fx: f32 = nx as f32;
    let fy: f32 = ny as f32;
    let fs: f32 = ns as f32;
    let mut img = ImageBuffer::new(nx, ny);

    let cam: Camera = Camera::new();

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    spheres.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    let world: World = World::from_vec(spheres);

    for j in 0..ny {
        for i in 0..nx {
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f32 = ((i as f32)+rand::random::<f32>())/fx;
                let v: f32 = (((ny-j) as f32)+rand::random::<f32>())/fy;
                let r: Ray = cam.get_ray(u, v);
                let p: Vec3 = r.point_at_parameter(2.0);
                col += color(r, &world);
            }
            col /= fx;
            let r: u8 = (255.99 * col.r()) as u8;
            let g: u8 = (255.99 * col.g()) as u8;
            let b: u8 = (255.99 * col.b()) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            img.put_pixel(i, j, pixel);
        }
    }
    let _ = img.save("assets/test.png");
}
