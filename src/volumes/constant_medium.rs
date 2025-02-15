use core::f64;

use crate::{
    bounding_box::BoundingBox,
    colour::RGB,
    hittable::{HitRecord, Hittable},
    material::Isotropic,
    ray::Ray,
    textures::texture::Texture,
    utilities::random,
    vec3d::Vec3d,
};

#[derive(Clone)]
pub struct ConstantMedium<THittable>
where
    THittable: Hittable,
{
    boundary: THittable,
    phase_function: Isotropic,
    neg_inv_density: f64,
}

impl<THittable> ConstantMedium<THittable>
where
    THittable: Hittable,
{
    pub fn new(
        boundary: THittable,
        texture: Box<dyn Texture + Sync>,
        density: f64,
    ) -> ConstantMedium<THittable> {
        let phase_function = Isotropic::new(texture);
        let neg_inv_density = -1.0 / density;

        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density,
        }
    }

    pub fn build_from_colour(
        boundary: THittable,
        colour: RGB,
        density: f64,
    ) -> ConstantMedium<THittable> {
        let phase_function = Isotropic::build_from_colour(colour);
        let neg_inv_density = -1.0 / density;

        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density,
        }
    }
}

impl<THittable: Hittable + Clone + Sync> Hittable for ConstantMedium<THittable> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Check if the ray hits the boundary anywhere on it's length
        let hr1 = self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY);
        if hr1.is_none() {
            return None;
        }
        let mut hr1 = hr1.unwrap();

        // Check that the ray passes through some non trivially small portion of
        // the boundary
        let hr2 = self.boundary.hit(ray, hr1.t + 0.0001, f64::INFINITY);
        if hr2.is_none() {
            return None;
        }
        let mut hr2 = hr2.unwrap();

        // Set t to min and max values
        if hr1.t < t_min {
            hr1.t = t_min;
        }
        if hr2.t > t_max {
            hr2.t = t_max;
        }

        // Check the ray intersects along the allowed portion of the ray
        if hr1.t >= hr2.t {
            return None;
        }

        // Check if the ray originates inside the boundary, in this case hr2 is
        // the point where the ray exists the boundary
        if hr1.t < 0.0 {
            hr1.t = 0.0;
        }

        let ray_length = ray.direction.len();
        let distance_inside_boundary = (hr2.t - hr1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hr1.t + hit_distance / ray_length;
        let point = ray.at(t);

        Some(HitRecord::new(
            point,
            Vec3d::new(1.0, 0.0, 0.0), // Arbitrary
            &self.phase_function,
            t,
            0.0,  // Arbitrary
            0.0,  // Arbitrary
            true, // Arbitrary
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox> {
        self.boundary.bounding_box(time0, time1)
    }
}
