use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{hit_record::HitRecord, hittable::Hittable},
    materials::Material,
    ray::Ray,
    vec3d::Point3d,
    vec3d::Vec3d,
};

/// Enum for the three axis-aligned rectangle structs.
#[derive(Debug, Clone, Copy)]
pub enum Rectangle<M: Material> {
    XY(RectangleXY<M>),
    XZ(RectangleXZ<M>),
    YZ(RectangleYZ<M>),
}

impl<M> Hittable for Rectangle<M>
where
    M: Material + Clone,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Rectangle::XY(rectangle_xy) => rectangle_xy.hit(ray, t_min, t_max),
            Rectangle::XZ(rectangle_xz) => rectangle_xz.hit(ray, t_min, t_max),
            Rectangle::YZ(rectangle_yz) => rectangle_yz.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox> {
        match self {
            Rectangle::XY(rectangle_xy) => rectangle_xy.bounding_box(time0, time1),
            Rectangle::XZ(rectangle_xz) => rectangle_xz.bounding_box(time0, time1),
            Rectangle::YZ(rectangle_yz) => rectangle_yz.bounding_box(time0, time1),
        }
    }
}

/// Axis-aligned rectangle for X-Y plane
#[derive(Debug, Clone, Copy)]
pub struct RectangleXY<M: Material> {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: M,
}

impl<M: Material> RectangleXY<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: M) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl<M> Hittable for RectangleXY<M>
where
    M: Material + Clone,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3d::new(0.0, 0.0, 1.0);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        Some(HitRecord::new(
            ray.at(t),
            normal,
            &self.material,
            t,
            u,
            v,
            front_face,
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        // The bounding box must have non-zero width in each dimension, so pad
        // the Z dimension a small amount.
        Some(BoundingBox::new(
            Point3d::new(self.x0, self.y0, self.k - 0.0001),
            Point3d::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

/// Axis-aligned rectangle for X-Z plane
#[derive(Debug, Clone, Copy)]
pub struct RectangleXZ<M: Material> {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: M,
}

impl<M: Material> RectangleXZ<M> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl<M> Hittable for RectangleXZ<M>
where
    M: Material + Clone,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3d::new(0.0, 1.0, 0.0);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        Some(HitRecord::new(
            ray.at(t),
            normal,
            &self.material,
            t,
            u,
            v,
            front_face,
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        // The bounding box must have non-zero width in each dimension, so pad
        // the Y dimension a small amount.
        Some(BoundingBox::new(
            Point3d::new(self.x0, self.k - 0.0001, self.z0),
            Point3d::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

/// Axis-aligned rectangle for Y-Z plane
#[derive(Debug, Clone, Copy)]
pub struct RectangleYZ<M: Material> {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: M,
}

impl<M: Material> RectangleYZ<M> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl<M> Hittable for RectangleYZ<M>
where
    M: Material + Clone,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3d::new(1.0, 0.0, 0.0);
        let (front_face, normal) = HitRecord::get_face_normal(ray, outward_normal);

        Some(HitRecord::new(
            ray.at(t),
            normal,
            &self.material,
            t,
            u,
            v,
            front_face,
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        // The bounding box must have non-zero width in each dimension, so pad
        // the X dimension a small amount.
        Some(BoundingBox::new(
            Point3d::new(self.k - 0.0001, self.y0, self.z0),
            Point3d::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
