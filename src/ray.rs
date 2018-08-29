
use vec3::*;

#[derive(Debug, PartialEq)]
pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {

    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray {
            a,
            b
        }
    }

    pub fn new_empty() -> Self {
        Ray {
            a: Vec3::new(0.0, 0.0, 0.0),
            b: Vec3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn origin(&self) -> Vec3 {
        Vec3::from_vec3(&self.a)
    }

    pub fn direction(&self) -> Vec3 {
        Vec3::from_vec3(&self.b)
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin() + t*self.direction()
    }
}