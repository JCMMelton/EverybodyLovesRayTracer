extern crate image;

use image::{ImageBuffer, Pixel, Rgb};
use std::f32;
use std::ops::{Add, Sub, Mul, Div};

struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {

    fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3{
            e: [e0, e1, e2]
        }
    }

    fn x(&self) -> f32 {
        self.e[0]
    }

    fn y(&self) -> f32 {
        self.e[1]
    }

    fn z(&self) -> f32 {
        self.e[2]
    }

    fn r(&self) -> f32 {
        self.e[0]
    }

    fn g(&self) -> f32 {
        self.e[1]
    }

    fn b(&self) -> f32 {
        self.e[2]
    }

    fn length(&self) -> f32 {
        f32::sqrt(f32::powf(e[0], 2.0) + f32::powf(e[1], 2.0) + f32::powf(e[2], 2.0))
    }

    fn squared_length(&self) -> f32 {
        f32::powf(e[0], 2.0) + f32::powf(e[1], 2.0) + f32::powf(e[2], 2.0)
    }

    fn make_unit_vector(&mut self) {
        let k: f32 = 1.0 / self.length();
        self.e[0] *= k;
        self.e[0] *= k;
        self.e[0] *= k;
    }

}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2]
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2]
        )
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2]
        )
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2]
        )
    }
}

struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    fn origin(&self) -> Vec3 {
        self.a
    }

    fn direction(&self) -> Vec3 {
        self.b
    }

    fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t*self.b
    }
}

fn main() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let fx: f32 = nx as f32;
    let fy: f32 = ny as f32;
    let mut img = ImageBuffer::new(nx, ny);
    for j in 0..ny-1 {
        for i in 0..nx {
            let rj:u32 = ny - j;
            let r: u8 = (255.99 * (i  as f32) / fx) as u8;
            let g: u8 = (255.99 * (rj as f32) / fy) as u8;
            let b: u8 = (255.99 * 0.2) as u8;
            let col: Vector3<u8> = Vector3::new(r, g, b);
            let pixel = Rgb::from_channels(col[0], col[1], col[2], 0);
            img.put_pixel(i, j, pixel);
        }
    }
    let _ = img.save("assets/test.png");
}
