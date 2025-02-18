use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{hit_record::HitRecord, hittable::Hittable},
    materials::Material,
    ray::Ray,
    utilities::get_sphere_uv,
    vec3d::Point3d,
    vec3d::Vec3d,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere<TMaterial>
where
    TMaterial: Material,
{
    center: Point3d,
    radius: f64,
    material: TMaterial,
}

impl<TMaterial> Sphere<TMaterial>
where
    TMaterial: Material,
    TMaterial: Clone,
{
    pub fn new(center: Point3d, radius: f64, material: TMaterial) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<TMaterial> Hittable for Sphere<TMaterial>
where
    TMaterial: Material + Sync + 'static,
    TMaterial: Clone,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
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
        let outward_normal = (point - self.center) / self.radius;
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

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            self.center - Vec3d::new(self.radius, self.radius, self.radius),
            self.center + Vec3d::new(self.radius, self.radius, self.radius),
        ))
    }
}
