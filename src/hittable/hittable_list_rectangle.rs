use crate::{
    bvh::bounding_box::BoundingBox, hittable::hittable::Hittable, materials::Material,
    objects::Rectangle, ray::Ray, utilities::surrounding_box,
};

use super::hit_record::HitRecord;

/// Stores a list of rectangles.
#[derive(Clone)]
pub struct HittableListRectangle<TMaterial>
where
    TMaterial: Material + Clone,
{
    time0: f64,
    time1: f64,
    pub items: Vec<Rectangle<TMaterial>>,
    bounding_box: Option<BoundingBox>,
}

impl<TMaterial> HittableListRectangle<TMaterial>
where
    TMaterial: Material + Clone,
{
    pub fn new(time0: f64, time1: f64) -> Self {
        Self {
            time0,
            time1,
            items: Vec::new(),
            bounding_box: None,
        }
    }

    pub fn add(&mut self, item: Rectangle<TMaterial>) {
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

    pub fn build(time0: f64, time1: f64, items: Vec<Rectangle<TMaterial>>) -> Self {
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

impl<TMaterial> Hittable for HittableListRectangle<TMaterial>
where
    TMaterial: Material + Clone,
{
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
