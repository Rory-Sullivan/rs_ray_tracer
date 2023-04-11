use crate::{
    colour::RGB,
    hittable::HitRecord,
    utilities::{random_unit_vec, reflect_vec},
    Ray,
};

pub trait Material {
    // Returns scattered ray and an attenuation colour
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)>;
}

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
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let mut scatter_direction = hit_record.normal + random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }
        let ray_out = Ray::new(hit_record.point, scatter_direction);
        return Some((ray_out, self.albedo));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: RGB,
}

impl Metal {
    pub fn new(albedo: RGB) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let reflected_direction = reflect_vec(&ray_in.direction.unit_vector(), &hit_record.normal);
        let reflected_ray = Ray::new(hit_record.point, reflected_direction);
        if reflected_ray.direction.dot(&hit_record.normal) > 0.0 {
            return Some((reflected_ray, self.albedo));
        }
        None
    }
}
