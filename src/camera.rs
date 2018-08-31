
use f32;
use utils::*;
use vec3::*;
use ray::*;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius: f32 = aperture/2.0;
        let theta: f32 = vfov*f32::consts::PI/180.0;
        let half_height: f32 = f32::tan(theta/2.0);
        let half_width:  f32 = aspect * half_height;
        let origin: Vec3 = lookfrom.copy();
        let w: Vec3 = Vec3::unit_vector(lookfrom - lookat);
        let u: Vec3 = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v: Vec3 = Vec3::cross(&w, &u);
        Camera {
            lower_left_corner: origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2.0 * half_width*focus_dist*u,
            vertical: 2.0 * half_height*focus_dist*v,
            origin,
            lens_radius,
            u,
            v,
            w
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * Utils::random_in_unit_disk();
        let offset: Vec3 = self.u*rd.x() + self.v*rd.y();
        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset
        )
    }
}