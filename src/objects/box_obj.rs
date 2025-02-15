use crate::{
    bounding_box::BoundingBox,
    hittable::{HitRecord, Hittable, HittableList},
    material::Material,
    objects::rectangle::{RectangleXY, RectangleXZ, RectangleYZ},
    ray::Ray,
    vec3d::Point3d,
};

/// And axis-aligned box made of 6 rectangles.
#[derive(Clone)]
pub struct BoxObj<'a> {
    box_min: Point3d,
    box_max: Point3d,
    sides: HittableList<'a>,
}

impl<'a> BoxObj<'a> {
    pub fn new<TMaterial>(box_min: Point3d, box_max: Point3d, material: TMaterial) -> Self
    where
        TMaterial: Material + Sync + 'static,
        TMaterial: Clone,
    {
        let mut sides = HittableList::<'a>::new();
        sides.add(Box::new(RectangleXY::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_min.z,
            material.clone(),
        )));
        sides.add(Box::new(RectangleXY::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_max.z,
            material.clone(),
        )));
        sides.add(Box::new(RectangleXZ::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_min.y,
            material.clone(),
        )));
        sides.add(Box::new(RectangleXZ::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_max.y,
            material.clone(),
        )));
        sides.add(Box::new(RectangleYZ::new(
            box_min.y,
            box_max.y,
            box_min.z,
            box_max.z,
            box_min.x,
            material.clone(),
        )));
        sides.add(Box::new(RectangleYZ::new(
            box_min.y, box_max.y, box_min.z, box_max.z, box_max.x, material,
        )));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl<'a> Hittable for BoxObj<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(self.box_min, self.box_max))
    }
}
