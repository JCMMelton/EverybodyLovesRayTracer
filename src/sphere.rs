
use vec3::*;
use hit::*;
use ray::*;
use material::*;
use bounding_box::*;
use utils::*;

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
    pub mat: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Material) -> Self {
        Sphere {
            center,
            radius,
            mat
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> (bool, HitRecord) {
        let oc: Vec3 = r.origin() - self.center.copy();
        let a: f32 = Vec3::dot(&r.direction(), &r.direction());
        let b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = Vec3::dot(&oc, &oc) - self.radius*self.radius;
        let discriminant: f32 = f32::powf(b, 2.0) - (a*c);
        let mut new_rec: HitRecord = HitRecord::new(self.mat);
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - f32::sqrt(f32::powf(b, 2.0) - a*c))/a;
            if temp < t_max && temp > t_min {
                new_rec.t = temp;
                new_rec.p = r.point_at_parameter(new_rec.t);
                new_rec.normal = (new_rec.p - self.center) / self.radius;
                new_rec.mat = self.mat;
                return (true, new_rec);
            }
            temp = (-b + f32::sqrt(f32::powf(b, 2.0) - a*c))/a;
            if temp < t_max && temp > t_min {
                new_rec.t = temp;
                new_rec.p = r.point_at_parameter(new_rec.t);
                new_rec.normal = (new_rec.p - self.center) / self.radius;
                new_rec.mat = self.mat;
                return (true, new_rec);
            }
        }
        new_rec.copy_from_hit_record(&rec);
        (false, new_rec)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        AABB::new(&(self.center.copy() - Vec3::from_value(self.radius)), &(self.center.copy() - Vec3::from_value(self.radius)))  
    }
    fn get_z_order(&self) -> u32 {
        let mut x: u32 = (self.center.x()*1.0) as u32;
        let mut y: u32 = (self.center.y()*1.0) as u32;
        let mut z: u32 = (self.center.z()*1.0) as u32;

        Utils::zeeify(x, y, z)
    }
    fn describe(&self) {
        println!("Sphere at {:?}", self.center);
    }
}
