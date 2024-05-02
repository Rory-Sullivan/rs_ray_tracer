use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    Point3d, Ray,
};

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
    TMaterial: Copy,
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
    TMaterial: Material + 'static,
    TMaterial: Copy,
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
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        Some(HitRecord::new(
            point,
            normal,
            Box::new(self.material),
            root,
            front_face,
        ))
    }
}
