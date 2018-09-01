
use vec3::*;
use material::*;
use ray::*;
use hit::*;
use bounding_box::*;

pub struct Rectangle {
    coords: [f32; 4],
    k: f32,
    mat: Material
}

impl Rectangle {
    pub fn new(coords: [f32; 4], k: f32, mat: Material) -> Self {
        Rectangle {
            coords,
            k,
            mat
        }
    }
    pub fn x0(&self) -> f32 {
        self.coords[0]
    }
    pub fn x1(&self) -> f32 {
        self.coords[1]
    }
    pub fn y0(&self) -> f32 {
        self.coords[2]
    }
    pub fn y1(&self) -> f32 {
        self.coords[3]
    }
}

impl Hit for Rectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> (bool, HitRecord) {
        let mut new_rec: HitRecord = HitRecord::new(Material::new_blank());
        let t: f32 = self.k - r.origin().z() / r.direction().z();
        if t < t_min || t > t_max {
            return (false, new_rec);
        }
        let x: f32 = r.origin().x() + t*r.direction().x();
        let y: f32 = r.origin().y() + t*r.direction().y();
        if x < self.x0() || x > self.x1() || y < self.y0() || y > self.y1() {
            return (false, new_rec);
        }
        // new_rec.u = (x-self.x0())/(self.x1()-self.x0());
        // new_rec.v = (y-self.y0())/(self.y1()-self.y0());
        new_rec.t = t;
        new_rec.mat = self.mat;
        new_rec.p = r.point_at_parameter(t);
        new_rec.normal = Vec3::new(0.0, 0.0, 1.0);
        (true, new_rec)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        AABB::new(&Vec3::from_value(0.0), &Vec3::from_value(0.0))
    }
}