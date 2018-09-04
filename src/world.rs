
use std::collections::BTreeMap;

use hit::*;
use ray::*;
use sphere::*;
use material::*;
use bounding_box::*;
use vec3::*;
//use bvh::*;

pub struct World {
    pub contents: Vec<Box<Hit>>,
    //pub tree: BTreeMap<u32, &Box<Hit>>
}

impl World {
    pub fn new() -> Self {
        World {
            contents: Vec::new(),
            // tree: BTreeMap::new()
        }
    }
    pub fn from_vec(contents: Vec<Box<Hit>>) -> Self {
        World {
            contents,
            // tree: BTreeMap::new()
        }
    }
    pub fn get_z_orders(&self) {
        let leaves: usize = self.contents.len();
        let nodes:  usize = leaves-1;
        // let mut top_node: BvhNode
        //let nodes: Vec<Box<BvhNode>> = Vec::new();
        for object in self.contents.iter() {
            println!("{:?}", object.get_z_order());
            println!("{:?}", format!("{:b}", object.get_z_order()) );
            // self.tree.entry(object.get_z_order()).or_insert(object);
            //nodes.push( Box::new(BvhNode::new(&object, object.get_z_order())) );
        }
        //println!("{:?}", nodes.len());
    }
    pub fn sphere_wall() -> Self {
        let mut contents: Vec<Box<Hit>> = Vec::new();
        let xm: usize = 10;
        let ym: usize = 10;
        for x in 0..xm {
            let xf: f32 = (x as f32)-((xm/2) as f32);
            for y in 0..ym {
                let yf: f32 = (y as f32)-((ym/2) as f32);
                contents.push(            
                    Box::new(
                        Sphere::new(
                            Vec3::new(xf, yf, -2.0), 
                            0.5,
                            Material::new(
                                Vec3::new(0.9, 0.4, 0.2),
                                MaterialComposition::Lambertian,
                                0.5,
                                0.0
                            )
                        )
                    ),
                );
            }
        }
        contents.push( 
            Box::new(           
                Sphere::new(
                    Vec3::new(0.0, 0.5, 2.5),
                    1.0,
                    Material::new(
                        Vec3::new(1.0, 1.0, 1.0),
                        MaterialComposition::DiffuseLight,
                        0.5,
                        0.0
                    )
                )
            )
        );
        World {
            contents
        }
    }
}

unsafe impl Sync for World {}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> (bool, HitRecord) {
        let mut temp_rec: HitRecord = HitRecord::new(Material::new_blank());
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;
        for object in self.contents.iter() {
            let obj_hit: (bool, HitRecord) = object.hit(&r, t_min, closest_so_far, &temp_rec);
            if obj_hit.0  {
                temp_rec = obj_hit.1;
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }
        (hit_anything, temp_rec)
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
    fn get_z_order(&self) -> u32 {
        0u32
    }
}