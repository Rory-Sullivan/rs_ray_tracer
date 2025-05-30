use crate::{
    colour::RGB,
    hittable::hit_record::HitRecord,
    ray::Ray,
    textures::{SolidColour, Texture},
    utilities::random_unit_vec,
};

use super::material::Material;

/// Lambertian reflectance is the property that defines an ideal "matte" or
/// diffusely reflecting surface. This material is very similar to the Diffuse
/// material but it allows for generic textures to be passed in instead of a
/// solid colour.
#[derive(Debug, Clone, Copy)]
pub struct Lambertian<Tex: Texture> {
    pub albedo: Tex,
}

impl<Tex: Texture> Lambertian<Tex> {
    pub fn new(albedo: Tex) -> Self {
        Lambertian { albedo }
    }
}

impl Lambertian<SolidColour> {
    pub fn build_from_colour(colour: RGB) -> Self {
        Lambertian::new(SolidColour::new(colour))
    }
}

impl<Tex: Texture> Material for Lambertian<Tex> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let mut scatter_direction = hit_record.normal + random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }
        let ray_out = Ray::new(hit_record.point, scatter_direction, ray_in.time);
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);
        Some((ray_out, attenuation))
    }
}
