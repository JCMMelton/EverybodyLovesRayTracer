
use vec3::*;
use hit::*;
use ray::*;
use material::*;
use bounding_box::*;

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
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center.copy();
        let a: f32 = Vec3::dot(&r.direction(), &r.direction());
        let b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = Vec3::dot(&oc, &oc) - self.radius*self.radius;
        let discriminant: f32 = f32::powf(b, 2.0) - (a*c);
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - f32::sqrt(f32::powf(b, 2.0) - a*c))/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat;
                return true;
            }
            temp = (-b + f32::sqrt(f32::powf(b, 2.0) - a*c))/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.mat;
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        AABB::new(&(self.center.copy() - Vec3::from_value(self.radius)), &(self.center.copy() - Vec3::from_value(self.radius)))  
    }
}
