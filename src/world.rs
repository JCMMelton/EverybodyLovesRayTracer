
use hit::*;
use ray::*;
use sphere::*;
use material::*;

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
}