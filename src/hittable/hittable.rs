use crate::{bvh::bounding_box::BoundingBox, ray::Ray};

use super::hit_record::HitRecord;

/// Trait for all objects that can be hit by a ray.
pub trait Hittable: DynClone {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox>;
}

/// Helper trait to make Box<dyn Hittable + Sync + '_> types clone-able. This is
/// necessary for clones that happen in the Bvh::build function.
///
/// Read more here https://quinedot.github.io/rust-learning/dyn-trait-clone.html
pub trait DynClone {
    fn dyn_clone<'a>(&self) -> Box<dyn Hittable + Sync + 'a>
    where
        Self: 'a;
}

impl<T: Clone + Hittable + Sync> DynClone for T {
    fn dyn_clone<'a>(&self) -> Box<dyn Hittable + Sync + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable + Sync + '_> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}
