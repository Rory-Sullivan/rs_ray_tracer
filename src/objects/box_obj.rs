use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{
        hit_record::HitRecord, hittable::Hittable, hittable_list_rectangle::HittableListRectangle,
    },
    materials::Material,
    objects::rectangle::{RectangleXY, RectangleXZ, RectangleYZ},
    ray::Ray,
    vec3d::Point3d,
};

use super::rectangle::Rectangle;

/// And axis-aligned box made of 6 rectangles.
#[derive(Clone)]
pub struct BoxObj<TMaterial>
where
    TMaterial: Material + Clone,
{
    box_min: Point3d,
    box_max: Point3d,
    sides: HittableListRectangle<TMaterial>,
}

impl<TMaterial> BoxObj<TMaterial>
where
    TMaterial: Material + Clone,
{
    pub fn new(box_min: Point3d, box_max: Point3d, material: TMaterial) -> Self {
        let mut sides = HittableListRectangle::new(0.0, 0.0);
        sides.add(Rectangle::XY(RectangleXY::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_min.z,
            material.clone(),
        )));
        sides.add(Rectangle::XY(RectangleXY::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_max.z,
            material.clone(),
        )));
        sides.add(Rectangle::XZ(RectangleXZ::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_min.y,
            material.clone(),
        )));
        sides.add(Rectangle::XZ(RectangleXZ::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_max.y,
            material.clone(),
        )));
        sides.add(Rectangle::YZ(RectangleYZ::new(
            box_min.y,
            box_max.y,
            box_min.z,
            box_max.z,
            box_min.x,
            material.clone(),
        )));
        sides.add(Rectangle::YZ(RectangleYZ::new(
            box_min.y, box_max.y, box_min.z, box_max.z, box_max.x, material,
        )));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl<TMaterial> Hittable for BoxObj<TMaterial>
where
    TMaterial: Material + Clone,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(self.box_min, self.box_max))
    }
}
