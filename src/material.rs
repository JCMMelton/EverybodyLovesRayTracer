
use vec3::*;
use ray::*;
use hit::*;
use utils::*;

#[derive(Debug, Copy, Clone)]
pub enum MaterialComposition {
    Lambertian,
    Metal
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub albedo: Vec3,
    pub composition: MaterialComposition
}

impl Material {

    pub fn new(albedo: Vec3, composition: MaterialComposition) -> Self {
        Material {
            albedo,
            composition
        }
    }
    pub fn new_blank() -> Self {
        Material {
            albedo: Vec3::new(0.0, 0.0, 0.0),
            composition: MaterialComposition::Lambertian
        }
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        match self.composition {
            MaterialComposition::Lambertian => self.scatter_lambertian(&r_in, &hit_record, &attenuation, &scattered),
            MaterialComposition::Metal => self.scatter_metal(&r_in, &hit_record, &attenuation, &scattered)
        }
    }

    pub fn scatter_lambertian(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        let target: Vec3 = hit_record.p + hit_record.normal + Utils::random_in_unit_sphere();
        let scatter: Ray = Ray::new(hit_record.p, target-hit_record.p);
        let atten: Vec3  = attenuation.copy() - self.albedo;
        (true, scatter, atten)
    }

    pub fn scatter_metal(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        let reflected: Vec3 = Utils::reflect(&Vec3::unit_vector(r_in.direction()), &hit_record.normal);
        let scatter: Ray = Ray::new(hit_record.p, reflected);
        let atten: Vec3  = self.albedo;
        (Vec3::dot(&scatter.direction(), &hit_record.normal) > 0.0, scatter, atten)
    }
}