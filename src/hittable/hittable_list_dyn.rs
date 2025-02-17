use crate::{
    bvh::bounding_box::BoundingBox, hittable::hittable::Hittable, ray::Ray,
    utilities::surrounding_box,
};

use super::hit_record::HitRecord;

#[derive(Clone)]
pub struct HittableListDyn<'a> {
    time0: f64,
    time1: f64,
    pub items: Vec<Box<dyn Hittable + Sync + 'a>>,
    bounding_box: Option<BoundingBox>,
}

impl<'a> HittableListDyn<'a> {
    pub fn new(time0: f64, time1: f64) -> Self {
        Self {
            time0,
            time1,
            items: Vec::new(),
            bounding_box: None,
        }
    }

    pub fn add(&mut self, item: Box<dyn Hittable + Sync + 'a>) {
        if self.bounding_box.is_none() {
            self.bounding_box = item.bounding_box(self.time0, self.time1);
        } else {
            self.bounding_box = Some(surrounding_box(
                self.bounding_box.unwrap(),
                item.bounding_box(self.time0, self.time1).unwrap(),
            ));
        }
        self.items.push(item);
    }

    pub fn build(time0: f64, time1: f64, items: Vec<Box<dyn Hittable + Sync + 'a>>) -> Self {
        let mut result = Self::new(time0, time1);
        for item in items {
            result.add(item);
        }
        result
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Hittable for HittableListDyn<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for item in self.items.iter() {
            match item.hit(ray, t_min, closest_so_far) {
                Some(hr) => {
                    closest_so_far = hr.t;
                    hit_record = Some(hr);
                }
                None => {}
            }
        }

        hit_record
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        self.bounding_box
    }
}
