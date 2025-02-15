use crate::{colour::RGB, hittable::HitRecord, ray::Ray, vec3d::Point3d};

pub trait Material {
    /// Returns scattered ray and an attenuation colour
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)>;

    /// Return the colour of emitted light. Defaults to black for non-emissive
    /// materials.
    fn emitted(&self, _u: f64, _v: f64, _p: Point3d) -> RGB {
        RGB(0.0, 0.0, 0.0)
    }
}
