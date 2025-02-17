use crate::{
    colour::RGB,
    hittable::hit_record::HitRecord,
    ray::Ray,
    utilities::{random, reflect_vec, refract_vec},
};

use super::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let new_direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                reflect_vec(&unit_direction, &hit_record.normal)
            } else {
                refract_vec(&unit_direction, &hit_record.normal, refraction_ratio)
            };

        Some((
            Ray::new(hit_record.point, new_direction, ray_in.time),
            RGB(1.0, 1.0, 1.0),
        ))
    }
}
