
use hit::*;
use ray::*;
use sphere::*;
use material::*;
use bounding_box::*;
use vec3::*;

pub struct World {
    pub contents: Vec<Sphere>
}

impl World {
    pub fn new() -> Self {
        World {
            contents: Vec::new()
        }
    }
    pub fn from_vec(contents: Vec<Sphere>) -> Self {
        World {
            contents
        }
    }
}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new(Material::new_blank());
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;
        for object in self.contents.iter() {
            if object.hit(&r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.copy_from_hit_record(&temp_rec);
            }
        }
        hit_anything
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