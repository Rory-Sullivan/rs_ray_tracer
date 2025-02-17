use crate::{
    colour::RGB,
    hittable::hit_record::HitRecord,
    ray::Ray,
    textures::{solid_colour::SolidColour, texture::Texture},
    vec3d::Point3d,
};

use super::material::Material;

#[derive(Clone)]
pub struct DiffuseLight {
    pub emit: Box<dyn Texture + Sync>,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture + Sync>) -> Self {
        DiffuseLight { emit }
    }

    pub fn build_from_colour(colour: RGB) -> Self {
        DiffuseLight::new(Box::new(SolidColour::new(colour)))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> RGB {
        self.emit.value(u, v, &p)
    }
}
