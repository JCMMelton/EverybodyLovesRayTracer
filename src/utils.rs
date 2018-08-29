
extern crate rand;

use vec3::*;

pub struct Utils {}

impl Utils {
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p: Vec3 = Vec3::new(0.0, 0.0, 0.0);
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
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v.copy() - 2.0*Vec3::dot(v, n)*n.copy()
    }
}