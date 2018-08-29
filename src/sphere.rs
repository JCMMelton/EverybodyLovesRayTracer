
use vec3::*;
use hit::*;
use ray::*;

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere {
            center,
            radius
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
                return true;
            }
            temp = (-b + f32::sqrt(f32::powf(b, 2.0) - a*c))/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}