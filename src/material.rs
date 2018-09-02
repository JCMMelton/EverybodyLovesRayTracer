
extern crate rand;


use vec3::*;
use ray::*;
use hit::*;
use utils::*;

#[derive(Debug, Copy, Clone)]
pub enum MaterialComposition {
    Lambertian,
    Metal,
    Dialectric,
    DiffuseLight
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub albedo: Vec3,
    pub composition: MaterialComposition,
    pub fuzz: f32,
    pub ref_idx: f32
}

impl Material {

    pub fn new(albedo: Vec3, composition: MaterialComposition, fuzz: f32, ref_idx: f32) -> Self {
        Material {
            albedo,
            composition,
            fuzz,
            ref_idx
        }
    }
    pub fn new_blank() -> Self {
        Material {
            albedo: Vec3::from_value(0.0),
            composition: MaterialComposition::Lambertian,
            fuzz: 1.0,
            ref_idx: 0.0
        }
    }

    pub fn emitted(&self) -> Vec3 {
        match self.composition {
            MaterialComposition::Lambertian => self.emit_default(),
            MaterialComposition::Metal => self.emit_default(),
            MaterialComposition::Dialectric => self.emit_default(),
            MaterialComposition::DiffuseLight => self.emit_diffuse_light()
        }
    }

    pub fn emit_default(&self) -> Vec3 {
        Vec3::from_value(0.0)
    }

    pub fn emit_diffuse_light(&self) -> Vec3 {
        self.albedo
    }

    pub fn scatter(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        match self.composition {
            MaterialComposition::Lambertian => self.scatter_lambertian(&r_in, &hit_record, &attenuation, &scattered),
            MaterialComposition::Metal => self.scatter_metal(&r_in, &hit_record, &attenuation, &scattered),
            MaterialComposition::Dialectric => self.scatter_dialectric(&r_in, &hit_record, &attenuation, &scattered),
            MaterialComposition::DiffuseLight => self.scatter_diffuse_light(&r_in, &hit_record, &attenuation, &scattered),
        }
    }

    pub fn scatter_diffuse_light(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        let scatter: Ray = Ray::new(r_in.origin(), r_in.direction());
        let atten: Vec3  = attenuation.copy();
        (false, scatter, atten)
    }

    pub fn scatter_lambertian(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        let target: Vec3 = hit_record.p + hit_record.normal + Utils::random_in_unit_sphere();
        let scatter: Ray = Ray::new(hit_record.p, target-hit_record.p);
        let atten: Vec3  = attenuation.copy() - self.albedo;
        (true, scatter, atten)
    }

    pub fn scatter_metal(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        let reflected: Vec3 = Utils::reflect(&Vec3::unit_vector(r_in.direction()), &hit_record.normal);
        let scatter: Ray = Ray::new(hit_record.p, reflected + self.fuzz*Utils::random_in_unit_sphere());
        let atten: Vec3  = self.albedo;
        (Vec3::dot(&scatter.direction(), &hit_record.normal) > 0.0, scatter, atten)
    }

    pub fn scatter_dialectric(&self, r_in: &Ray, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) -> (bool, Ray, Vec3) {
        let outward_normal: Vec3;
        let reflected: Vec3 = Utils::reflect(&r_in.direction(), &hit_record.normal);
        let mut refracted: Vec3 = Vec3::from_value(0.0);
        let ni_over_nt: f32;
        let atten: Vec3  = Vec3::from_value(1.0);
        let reflect_prob: f32;
        let cosine: f32;
        let mut scatter: Ray;

        if Vec3::dot(&r_in.direction(), &hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3::dot(&r_in.direction(), &hit_record.normal) / r_in.direction().length();
        }
        else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -Vec3::dot(&r_in.direction(), &hit_record.normal) / r_in.direction().length();
        }
        if Utils::refract(&r_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = Utils::schlick(cosine, self.ref_idx);
        }
        else {
            scatter = Ray::new(hit_record.p, reflected);
            reflect_prob = 1.0;
        }
        if rand::random::<f32>() < reflect_prob {
            scatter = Ray::new(hit_record.p, reflected);
        }
        else {
            scatter = Ray::new(hit_record.p, refracted);
        }
        (true, scatter, atten)
    }
}