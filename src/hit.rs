
use vec3::*;
use ray::*;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord{
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0)
        }
    }
    pub fn from_hit_record(rec: &HitRecord) -> Self {
        HitRecord{
            t: rec.t,
            p: rec.p,
            normal: rec.normal
        }
    }
    pub fn copy_from_hit_record(&mut self, rec: &HitRecord) {
        self.t = rec.t;
        self.p = rec.p;
        self.normal = rec.normal;
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

