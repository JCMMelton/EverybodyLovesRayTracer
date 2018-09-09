
use vec3::*;
use ray::*;
use hit::*;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn new(a: &Vec3, b: &Vec3) -> Self {
        AABB {
            min: a.copy(),
            max: b.copy()
        }
    }
    pub fn min(&self) -> Vec3 {
        self.min
    }
    pub fn max(&self) -> Vec3 {
        self.max
    }
    pub fn hit1(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let t0: f32 = f32::min(
                (self.min[a] - r.origin()[a]) / r.direction()[a],
                 self.max[a] - r.origin()[a] / r.direction()[a]);
            let t1: f32 = f32::max(
                (self.min[a] - r.origin()[a]) / r.direction()[a],
                 self.max[a] - r.origin()[a] / r.direction()[a]);
            let tmin_l = f32::max(t0, tmin);
            let tmax_l = f32::min(t1, tmax);
            if tmax_l <= tmin_l {
                return false;
            }
        }
        true
    }
    pub fn hit2(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d: f32 = 1.0/r.direction()[a];
            let mut t0: f32 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1: f32 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                let temp: f32 = t0;
                t0 = t1;
                t1 = temp;
            }
            let tmin_l = if t0 > tmin { t0 } else { tmin };
            let tmax_l = if t1 < tmin { t1 } else { tmax };
            if tmax_l < tmin_l {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(box1: AABB, box2: AABB) -> AABB {
        let small: Vec3 = Vec3::new(
            f32::min(box1.min().x(), box2.min().x()),
            f32::min(box1.min().y(), box2.min().y()),
            f32::min(box1.min().z(), box2.min().z())
        );
        let big:   Vec3 = Vec3::new(
            f32::max(box1.max().x(), box2.max().x()),
            f32::max(box1.max().y(), box2.max().y()),
            f32::max(box1.max().z(), box2.max().z())
        );
        AABB{
            min: small, 
            max: big
        }
    }
}

impl Hit for AABB {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> (bool, HitRecord) {
        for a in 0..3 {
            let t0: f32 = f32::min(
                (self.min[a] - r.origin()[a]) / r.direction()[a],
                 self.max[a] - r.origin()[a] / r.direction()[a]);
            let t1: f32 = f32::max(
                (self.min[a] - r.origin()[a]) / r.direction()[a],
                 self.max[a] - r.origin()[a] / r.direction()[a]);
            let tmin_l = f32::max(t0, t_min);
            let tmax_l = f32::min(t1, t_max);
            if tmax_l <= tmin_l {
                return (false, HitRecord::from_hit_record(rec) );
            }
        }
        (true, HitRecord::from_hit_record(rec) )
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        AABB::new(&self.min, &self.max)
    }
    fn get_z_order(&self) -> u32 {
        0u32
    }
    fn describe(&self) {
        println!("AABB at {:?}{:?}", self.min(), self.max());
    }
}