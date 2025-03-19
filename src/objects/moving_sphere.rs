use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{hit_record::HitRecord, hittable::Hittable},
    materials::Material,
    ray::Ray,
    utilities::{get_sphere_uv, surrounding_box},
    vec3d::Point3d,
    vec3d::Vec3d,
};

#[derive(Debug, Clone, Copy)]
pub struct MovingSphere<M: Material> {
    center0: Point3d,
    center1: Point3d,
    time0: f64,
    time1: f64,
    radius: f64,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Point3d,
        center1: Point3d,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3d {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M> Hittable for MovingSphere<M>
where
    M: Material + Clone + Sync,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.len_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let d_sqrt = discriminant.sqrt();
        let mut root = (-half_b - d_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + d_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center(ray.time)) / self.radius;
        let (u, v) = get_sphere_uv(outward_normal);
        let (front_face, normal) = HitRecord::get_face_normal(ray, outward_normal);

        Some(HitRecord::new(
            point,
            normal,
            &self.material,
            root,
            u,
            v,
            front_face,
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox> {
        let radius_vec = Vec3d::new(self.radius, self.radius, self.radius);
        let box0 = BoundingBox::new(
            self.center(time0) - radius_vec,
            self.center(time0) + radius_vec,
        );
        let box1 = BoundingBox::new(
            self.center(time1) - radius_vec,
            self.center(time1) + radius_vec,
        );

        Some(surrounding_box(box0, box1))
    }
}
