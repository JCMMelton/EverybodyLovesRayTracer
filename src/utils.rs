
extern crate rand;

use vec3::*;

pub struct Utils {}

impl Utils {

    pub fn tozee(n: u32) -> u32 {
        let mut m: u32;
        m = (n | (n << 8)) & 0x00FF00FF;
        m = (m | (m << 4)) & 0x0F0F0F0F;
        m = (m | (m << 2)) & 0x33333333;
        m = (m | (m << 1)) & 0x55555555;
        m
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut p: Vec3;
        loop {
                p = 2.0*Vec3::new(
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    0.0
                ) - Vec3::new(1.0, 1.0, 0.0);
            if Vec3::dot(&p, &p) >= 1.0 {
                break;
            } 
        }
        p
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p: Vec3;
        loop {
            p = 2.0*Vec3::new(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>()
            ) - Vec3::new(1.0, 1.0, 1.0);
            if p.squared_length() >= 1.0 {
                break;
            }
        }
        p
    }

    pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let r0: f32 = (1.0-ref_idx) / (1.0+ref_idx);
        f32::powf(r0, 2.0) + (1.0-r0) * f32::powf(1.0-cosine, 5.0)
    }

    pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
        let uv: Vec3 = Vec3::unit_vector(v.copy());
        let dt: f32 = Vec3::dot(&uv, n);
        let discriminant: f32 = 1.0 - f32::powf(ni_over_nt, 2.0)*(1.0-f32::powf(dt, 2.0));
        if discriminant > 0.0 {
            *refracted = ni_over_nt * (uv.copy() - n.copy()*dt) - n.copy()*f32::sqrt(discriminant);
            return true;
        }
        false
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v.copy() - 2.0*Vec3::dot(v, n)*n.copy()
    }

}