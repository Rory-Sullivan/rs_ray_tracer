use crate::vec3d::{Point3d, Vec3d};

pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3d, direction: Vec3d, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3d {
        self.origin + (t * self.direction)
    }
}
