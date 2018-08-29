extern crate image;

use image::{ImageBuffer, Pixel, Rgb};
use std::f32;

mod vec3;
use vec3::*;

#[derive(Debug, PartialEq)]
struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {

    fn new(a: Vec3, b: Vec3) -> Self {
        Ray {
            a,
            b
        }
    }

    fn origin(&self) -> Vec3 {
        Vec3::from_vec3(&self.a)
    }

    fn direction(&self) -> Vec3 {
        Vec3::from_vec3(&self.b)
    }

    fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin() + t*self.direction()
    }
}

fn hit_sphere(center: &Vec3, radius: &f32, r: &Ray) -> bool {
    let oc: Vec3 = r.origin() - center.copy();
    let a: f32 = Vec3::dot(&r.direction(), &r.direction());
    let b: f32 = 2.0 * Vec3::dot(&oc, &r.direction());
    let c: f32 = Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant: f32 = f32::powf(b, 2.0) - (4.0*a*c);
    discriminant > 0.0
}

fn color(ray: Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), &0.5, &ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = Vec3::unit_vector(ray.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let fx: f32 = nx as f32;
    let fy: f32 = ny as f32;
    let mut img = ImageBuffer::new(nx, ny);
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3    = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3      = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3        = Vec3::new(0.0, 0.0, 0.0);
    for j in 0..ny-1 {
        for i in 0..nx {
            let u: f32 = (i as f32)/fx;
            let v: f32 = ((ny-j) as f32)/fy;
            let r: Ray = Ray::new(
                origin.copy(),
                lower_left_corner.copy() + (u*horizontal.copy()) + (v*vertical.copy())
            );
            let col: Vec3 = color(r);
            let r: u8 = (255.99 * col.r()) as u8;
            let g: u8 = (255.99 * col.g()) as u8;
            let b: u8 = (255.99 * col.b()) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            img.put_pixel(i, j, pixel);
        }
    }
    let _ = img.save("assets/test.png");
}
