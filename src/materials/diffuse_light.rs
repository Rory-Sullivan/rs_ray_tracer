use crate::{
    colour::RGB,
    hittable::hit_record::HitRecord,
    ray::Ray,
    textures::{SolidColour, Texture},
    vec3d::Point3d,
};

use super::material::Material;

#[derive(Clone)]
pub struct DiffuseLight<TTexture>
where
    TTexture: Texture + Sync,
{
    pub emit: TTexture,
}

impl<TTexture> DiffuseLight<TTexture>
where
    TTexture: Texture + Sync,
{
    pub fn new(emit: TTexture) -> Self {
        DiffuseLight { emit }
    }
}

impl DiffuseLight<SolidColour> {
    pub fn build_from_colour(colour: RGB) -> Self {
        DiffuseLight::new(SolidColour::new(colour))
    }
}

impl<TTexture> Material for DiffuseLight<TTexture>
where
    TTexture: Texture + Sync,
{
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> RGB {
        self.emit.value(u, v, &p)
    }
}
