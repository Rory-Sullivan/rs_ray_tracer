use crate::{
    colour::RGB,
    hittable::hit_record::HitRecord,
    ray::Ray,
    textures::{SolidColour, Texture},
    utilities::random_vec_in_unit_sphere,
};

use super::material::Material;

/// An isotropic material that scatters rays in a random direction, used for
/// volumes like fog and smoke.
#[derive(Clone)]
pub struct Isotropic<TTexture>
where
    TTexture: Texture + Sync,
{
    pub albedo: TTexture,
}

impl<TTexture> Isotropic<TTexture>
where
    TTexture: Texture + Sync,
{
    pub fn new(albedo: TTexture) -> Self {
        Isotropic { albedo }
    }
}
impl Isotropic<SolidColour> {
    pub fn build_from_colour(colour: RGB) -> Self {
        Isotropic::new(SolidColour::new(colour))
    }
}

impl<TTexture> Material for Isotropic<TTexture>
where
    TTexture: Texture + Sync,
{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let scattered = Ray::new(hit_record.point, random_vec_in_unit_sphere(), ray_in.time);
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);

        Some((scattered, attenuation))
    }
}
