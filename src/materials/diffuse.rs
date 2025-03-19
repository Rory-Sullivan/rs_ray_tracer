use crate::{colour::RGB, hittable::hit_record::HitRecord, ray::Ray, utilities::random_unit_vec};

use super::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Diffuse {
    pub albedo: RGB,
}

impl Diffuse {
    pub fn new(albedo: RGB) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let mut scatter_direction = hit_record.normal + random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }
        let ray_out = Ray::new(hit_record.point, scatter_direction, ray_in.time);
        Some((ray_out, self.albedo))
    }
}
