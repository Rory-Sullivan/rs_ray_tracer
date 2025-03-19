use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{hit_record::HitRecord, hittable::Hittable},
    ray::Ray,
    vec3d::Vec3d,
};

/// A translate trait to handle "moving" a hittable object. Does not actually
/// move the object but rather updates the hit function to "move" the ray before
/// passing it to the objects hit function.
#[derive(Clone)]
pub struct Translate<H: Hittable> {
    offset: Vec3d,
    object: H,
}

impl<H: Hittable> Translate<H> {
    pub fn new(offset: Vec3d, object: H) -> Self {
        Self { offset, object }
    }
}

impl<H: Hittable + Clone> Hittable for Translate<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        match self.object.hit(&moved_ray, t_min, t_max) {
            Some(hr) => {
                let (front_face, normal) = HitRecord::get_face_normal(&moved_ray, hr.normal);
                Some(HitRecord::new(
                    hr.point + self.offset,
                    normal,
                    hr.material,
                    hr.t,
                    hr.u,
                    hr.v,
                    front_face,
                ))
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox> {
        match self.object.bounding_box(time0, time1) {
            Some(bb) => Some(BoundingBox::new(bb.min + self.offset, bb.max + self.offset)),
            None => None,
        }
    }
}
