use crate::{Point3d, Vec3d};

pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
}

impl Ray {
    pub fn new(origin: Point3d, direction: Vec3d) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Point3d {
        self.origin + (t * self.direction)
    }
}
