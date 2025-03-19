use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{hit_record::HitRecord, hittable::Hittable},
    ray::Ray,
    utilities::degrees_to_radians,
    vec3d::{Point3d, Vec3d},
};

/// A rotation instance to handle "rotating" a hittable object around the
/// x-axis. Does not actually rotate the object but rather updates the hit
/// function to "rotate" the ray before passing it to the objects hit function.
#[derive(Clone)]
pub struct RotateX<H: Hittable> {
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<BoundingBox>,
    object: H,
}

impl<H: Hittable> RotateX<H> {
    pub fn new(angle: f64, object: H, t0: f64, t1: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        match object.bounding_box(t0, t1) {
            Some(obj_bb) => {
                let mut min = Point3d::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Point3d::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = (i as f64) * obj_bb.max.x + ((1 - i) as f64) * obj_bb.min.x;
                            let y = (j as f64) * obj_bb.max.y + ((1 - j) as f64) * obj_bb.min.y;
                            let z = (k as f64) * obj_bb.max.z + ((1 - k) as f64) * obj_bb.min.z;

                            let new_y = -sin_theta * z + cos_theta * y;
                            let new_z = cos_theta * z + sin_theta * y;

                            let temp_vec = Vec3d::new(x, new_y, new_z);
                            min.x = crate::utilities::min(min.x, temp_vec.x);
                            min.y = crate::utilities::min(min.y, temp_vec.y);
                            min.z = crate::utilities::min(min.z, temp_vec.z);

                            max.x = crate::utilities::max(max.x, temp_vec.x);
                            max.y = crate::utilities::max(max.y, temp_vec.y);
                            max.z = crate::utilities::max(max.z, temp_vec.z);
                        }
                    }
                }

                let bounding_box = BoundingBox::new(min, max);
                Self {
                    sin_theta,
                    cos_theta,
                    bounding_box: Some(bounding_box),
                    object,
                }
            }
            None => Self {
                sin_theta,
                cos_theta,
                bounding_box: None,
                object,
            },
        }
    }
}

impl<H: Hittable + Clone> Hittable for RotateX<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Vec3d::new(
            ray.origin.x,
            self.sin_theta * ray.origin.z + self.cos_theta * ray.origin.y,
            self.cos_theta * ray.origin.z - self.sin_theta * ray.origin.y,
        );
        let direction = Vec3d::new(
            ray.direction.x,
            self.sin_theta * ray.direction.z + self.cos_theta * ray.direction.y,
            self.cos_theta * ray.direction.z - self.sin_theta * ray.direction.y,
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        match self.object.hit(&rotated_ray, t_min, t_max) {
            Some(hr) => {
                let point = Point3d::new(
                    hr.point.x,
                    -self.sin_theta * hr.point.z + self.cos_theta * hr.point.y,
                    self.cos_theta * hr.point.z + self.sin_theta * hr.point.y,
                );
                let temp_normal = Point3d::new(
                    hr.normal.x,
                    -self.sin_theta * hr.normal.z + self.cos_theta * hr.normal.y,
                    self.cos_theta * hr.normal.z + self.sin_theta * hr.normal.y,
                );
                let (front_face, normal) = HitRecord::get_face_normal(&rotated_ray, temp_normal);

                Some(HitRecord::new(
                    point,
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

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        self.bounding_box
    }
}
