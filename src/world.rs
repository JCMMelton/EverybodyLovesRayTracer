
use hit::*;
use ray::*;
use material::*;
use bounding_box::*;
use vec3::*;

pub struct World {
    pub contents: Vec<Box<Hit>>
}

impl World {
    pub fn new() -> Self {
        World {
            contents: Vec::new()
        }
    }
    pub fn from_vec(contents: Vec<Box<Hit>>) -> Self {
        World {
            contents
        }
    }
}

unsafe impl Sync for World {}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> (bool, HitRecord) {
        let mut temp_rec: HitRecord = HitRecord::new(Material::new_blank());
        let mut new_rec: HitRecord = HitRecord::new(Material::new_blank());
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;
        for object in self.contents.iter() {
            let obj_hit = object.hit(&r, t_min, closest_so_far, &temp_rec);
            if obj_hit.0  {
                temp_rec = obj_hit.1;
                hit_anything = true;
                closest_so_far = temp_rec.t;
                new_rec.copy_from_hit_record(&temp_rec);
            }
        }
        (hit_anything, new_rec)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        let mut temp_box: AABB = AABB::new(&Vec3::from_value(0.0), &Vec3::from_value(0.0));
        if self.contents.len() < 1 {
            return temp_box;
        }
        for object in self.contents.iter() {
            temp_box = AABB::surrounding_box(object.bounding_box(t0, t1), temp_box);
        }
        temp_box
    }
}