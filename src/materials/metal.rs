use crate::{
    colour::RGB,
    hittable::HitRecord,
    ray::Ray,
    utilities::{random_vec_in_unit_sphere, reflect_vec},
};

use super::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: RGB,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: RGB, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let reflected_direction = reflect_vec(&ray_in.direction.unit_vector(), &hit_record.normal)
            + self.fuzz * random_vec_in_unit_sphere();
        let reflected_ray = Ray::new(hit_record.point, reflected_direction, ray_in.time);
        if reflected_ray.direction.dot(&hit_record.normal) > 0.0 {
            return Some((reflected_ray, self.albedo));
        }
        None
    }
}
