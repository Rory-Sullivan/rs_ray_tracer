use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{hit_record::HitRecord, hittable::Hittable},
    ray::Ray,
};

/// A scale instance to handle "scaling" a hittable object. Does not actually
/// scale the object but rather updates the hit function to "scale" the ray
/// before passing it to the objects hit function.
#[derive(Clone)]
pub struct Scale {
    x: f64,
    y: f64,
    z: f64,
    object: Box<dyn Hittable>,
}

impl Scale {
    pub fn new(x: f64, y: f64, z: f64, object: Box<dyn Hittable>) -> Self {
        Self { x, y, z, object }
    }
}

impl Hittable for Scale {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let scaled_ray = Ray::new(
            ray.origin.scale(1.0 / self.x, 1.0 / self.y, 1.0 / self.z),
            ray.direction,
            ray.time,
        );

        match self.object.hit(&scaled_ray, t_min, t_max) {
            Some(hr) => {
                let (front_face, normal) = HitRecord::get_face_normal(&scaled_ray, hr.normal);
                Some(HitRecord::new(
                    hr.point.scale(self.x, self.y, self.z),
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
            Some(bb) => Some(BoundingBox::new(
                bb.min.scale(self.x, self.y, self.z),
                bb.max.scale(self.x, self.y, self.z),
            )),
            None => None,
        }
    }
}
