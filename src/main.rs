extern crate image;
extern crate rand;

use image::{ImageBuffer, Pixel, Rgb};
use std::f32;

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

fn color(ray: Ray, world: &World, depth: i32) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new(
        Material::new_blank()
    );
    if world.hit(&ray, 0.0001, f32::MAX, &mut hit_record) {
        let mut scattered: Ray = Ray::new_empty();
        let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let scatter_result: (bool, Ray, Vec3) = hit_record.mat.scatter(&ray, &hit_record, &attenuation, &scattered);
        if depth < 50 && scatter_result.0 {
            return scatter_result.2*color(scatter_result.1, &world, depth+1);
        }
        else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }
    else {
        let unit_direction: Vec3 = Vec3::unit_vector(ray.direction());
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
    }
}

fn sphere_swoop(n: u32) -> Vec<Sphere> {
    let mut spheres: Vec<Sphere> = Vec::new();
    for s in 0..n {
        let sf: f32 = s as f32;
        spheres.push(
            Sphere::new(
                Vec3::new(0.0+f32::sin(sf), 0.0+f32::cos(sf), -1.0-(sf*0.5)), 0.5+(sf*0.1),
                Material::new(
                    Vec3::new(0.9, 0.9, 0.9),
                    MaterialComposition::Metal,
                    0.1,
                    0.0
                )
            )
        );
    }
    spheres
}

fn main() {
    let nx: u32 = 600;
    let ny: u32 = nx/2;
    let ns: u32 = nx/2;
    let fx: f32 = nx as f32;
    let fy: f32 = ny as f32;
    let fs: f32 = ns as f32;
    let mut img = ImageBuffer::new(nx, ny);

    let cam: Camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        fx/fy
    );

    let mut spheres: Vec<Sphere> = Vec::new();//sphere_swoop(20);
    spheres.push(
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0), 0.5,
            Material::new(
                Vec3::new(0.9, 0.9, 0.9),
                MaterialComposition::Dialectric,
                0.2,
                0.9
            )
        )
    );
    spheres.push(
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
             Material::new(
                 Vec3::new(0.8, 0.8, 0.3),
                 MaterialComposition::Lambertian,
                 0.0,
                 0.0
             )
        )
    );
    spheres.push(
        Sphere::new(
            Vec3::new(1.4, 0.2, -1.2),
            0.5,
             Material::new(
                 Vec3::new(0.9, 0.9, 0.9),
                 MaterialComposition::Metal,
                 0.0,
                 0.0
             )
        )
    );
    spheres.push(
        Sphere::new(
            Vec3::new(-2.0, 3.0, -3.0),
            1.75,
             Material::new(
                 Vec3::new(0.8, 0.8, 0.8),
                 MaterialComposition::Metal,
                 0.0,
                 0.0
             )
        )
    );
    spheres.push(
        Sphere::new(
            Vec3::new(0.0, 2.0, 3.0),
            1.75,
             Material::new(
                 Vec3::new(0.8, 0.8, 0.8),
                 MaterialComposition::Lambertian,
                 0.0,
                 0.0
             )
        )
    );
    let world: World = World::from_vec(spheres);
    let depth: i32 = 0;
    for j in 0..ny {
        for i in 0..nx {
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u: f32 = ((i as f32)+rand::random::<f32>())/fx;
                let v: f32 = (((ny-j) as f32)+rand::random::<f32>())/fy;
                let r: Ray = cam.get_ray(u, v);
                let p: Vec3 = r.point_at_parameter(2.0);
                col += color(r, &world, depth);
            }
            col /= fx;
            col = Vec3::new(f32::sqrt(col.r()),f32::sqrt(col.g()),f32::sqrt(col.b()) );
            let r: u8 = (255.99 * col.r()) as u8;
            let g: u8 = (255.99 * col.g()) as u8;
            let b: u8 = (255.99 * col.b()) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            img.put_pixel(i, j, pixel);
        }
    }
    let _ = img.save("assets/test.png");
}
