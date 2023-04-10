use crate::{Point3d, Ray, Vec3d};

pub struct Camera {
    origin: Point3d,
    horizontal: Vec3d,
    vertical: Vec3d,
    lower_left_corner: Point3d,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;
        let focal_length: f64 = 1.0;

        let origin = Point3d::new(0.0, 0.0, 0.0);
        let horizontal = Vec3d::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3d::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3d::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        // assert!(0.0 <= u, "Invalid value for u: {u}");
        // assert!(u < 1.0, "Invalid value for u: {u}");
        // assert!(0.0 <= v, "Invalid value for v: {v}");
        // assert!(v < 1.0, "Invalid value for v: {v}");

        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
