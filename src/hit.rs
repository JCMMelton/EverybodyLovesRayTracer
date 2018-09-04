
use vec3::*;
use ray::*;
use material::*;
use bounding_box::*;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Material,
    // pub u: f32,
    // pub v: f32
}

impl HitRecord {
    pub fn new(mat: Material) -> Self {
        HitRecord{
            t: 0.0,
            p: Vec3::from_value(0.0),
            normal: Vec3::from_value(0.0),
            mat
        }
    }
    pub fn from_hit_record(rec: &HitRecord) -> Self {
        HitRecord{
            t: rec.t,
            p: rec.p,
            normal: rec.normal,
            mat: rec.mat
        }
    }
    pub fn copy_from_hit_record(&mut self, rec: &HitRecord) {
        self.t = rec.t;
        self.p = rec.p;
        self.normal = rec.normal;
        self.mat = rec.mat;
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> (bool, HitRecord);
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB;
    fn get_z_order(&self) -> u32;
}