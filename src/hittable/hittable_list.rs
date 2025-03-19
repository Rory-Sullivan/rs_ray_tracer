use std::sync::Arc;

use crate::{
    bvh::bounding_box::BoundingBox, hittable::hittable::Hittable, ray::Ray,
    utilities::surrounding_box_option,
};

use super::hit_record::HitRecord;

/// Stores a list of hittable objects. Uses dynamic trait objects to allow for
/// any struct that implements the Hittable trait to be a part of the list.
#[derive(Clone)]
pub struct HittableList {
    time0: f64,
    time1: f64,
    items: Arc<[Box<dyn Hittable>]>,
    bounding_box: Option<BoundingBox>,
}

impl HittableList {
    pub fn build(time0: f64, time1: f64, items: &[Box<dyn Hittable>]) -> Self {
        // Get bounding box of all items and collect items into `Arc<[Box<dyn Hittable>]>`
        let mut bounding_box: Option<BoundingBox> = None;
        for item in items.iter() {
            bounding_box = surrounding_box_option(bounding_box, item.bounding_box(time0, time1))
        }
        let items = Arc::from(items);

        Self {
            time0,
            time1,
            items,
            bounding_box,
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for item in self.items.iter() {
            if let Some(hr) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
        }

        hit_record
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        self.bounding_box
    }
}

impl std::fmt::Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HittableList")
            .field("time0", &self.time0)
            .field("time1", &self.time1)
            .field(
                "items",
                &format_args!("Arc<[Box<dyn Hittable>]>[{}]", self.items.len()),
            )
            .field("bounding_box", &self.bounding_box)
            .finish()
    }
}
