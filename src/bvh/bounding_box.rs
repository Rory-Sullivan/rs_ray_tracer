use crate::{ray::Ray, vec3d::Point3d};

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Point3d,
    pub max: Point3d,
    longest_axis: usize,
}

impl BoundingBox {
    pub fn new(min: Point3d, max: Point3d) -> Self {
        if min.x > max.x || min.y > max.y || min.z > max.z {
            panic!("min must be less than max; min: {min:?}, max: {max:?}")
        }
        let len_x = max.x - min.x;
        let len_y = max.y - min.y;
        let len_z = max.z - min.z;
        let longest_axis = if len_x >= len_y && len_x >= len_z {
            0
        } else if len_y >= len_z {
            1
        } else {
            2
        };

        BoundingBox {
            min,
            max,
            longest_axis,
        }
    }

    /// Returns whether or not a ray intersects with a bounding box, see notes
    /// for why this works.
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        // Iterate over 3 dimensions, x, y, and z
        for i in 0..3 {
            let inv_dir = 1.0 / ray.direction.get_axis(i);
            let ray_origin_i = ray.origin.get_axis(i);

            let (t0, t1) = if inv_dir >= 0.0 {
                (
                    (self.min.get_axis(i) - ray_origin_i) * inv_dir,
                    (self.max.get_axis(i) - ray_origin_i) * inv_dir,
                )
            } else {
                // Swap t0 and t1
                (
                    (self.max.get_axis(i) - ray_origin_i) * inv_dir,
                    (self.min.get_axis(i) - ray_origin_i) * inv_dir,
                )
            };

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }

    pub fn longest_axis(&self) -> usize {
        self.longest_axis
    }
}

#[cfg(test)]
mod bounding_box_tests {
    use std::f64;

    use crate::vec3d::Vec3d;

    use super::*;

    #[test]
    fn hit_should_return_true_when_ray_hits() {
        let b_box = BoundingBox::new(Vec3d::new(1.0, 1.0, 1.0), Vec3d::new(3.0, 3.0, 3.0));
        let ray = Ray::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(1.0, 1.0, 1.0), 0.0);

        let result = b_box.hit(&ray, 0.0, f64::MAX);

        assert_eq!(result, true);
    }

    #[test]
    fn hit_should_return_false_when_ray_misses_completely() {
        let b_box = BoundingBox::new(Vec3d::new(1.0, 1.0, 1.0), Vec3d::new(3.0, 3.0, 3.0));
        let ray = Ray::new(Vec3d::new(3.0, 0.0, 0.0), Vec3d::new(1.0, 1.0, 1.0), 0.0);

        let result = b_box.hit(&ray, 0.0, f64::MAX);

        assert_eq!(result, false);
    }

    #[test]
    fn hit_should_return_false_when_ray_hits_not_inside_ray_segment() {
        let b_box = BoundingBox::new(Vec3d::new(1.0, 1.0, 1.0), Vec3d::new(3.0, 3.0, 3.0));
        let ray = Ray::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(-1.0, -1.0, -1.0), 0.0);

        let result = b_box.hit(&ray, 0.0, f64::MAX);

        assert_eq!(result, false);
    }

    #[test]
    fn hit_should_return_true_when_ray_parallel_to_face_and_hits() {
        let b_box = BoundingBox::new(Vec3d::new(1.0, 1.0, 1.0), Vec3d::new(3.0, 3.0, 3.0));
        let ray = Ray::new(Vec3d::new(1.5, 1.5, 0.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = b_box.hit(&ray, 0.0, f64::MAX);

        assert_eq!(result, true);
    }

    #[test]
    fn hit_should_return_false_when_ray_parallel_to_face_and_misses() {
        let b_box = BoundingBox::new(Vec3d::new(1.0, 1.0, 1.0), Vec3d::new(3.0, 3.0, 3.0));
        let ray = Ray::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = b_box.hit(&ray, 0.0, f64::MAX);

        assert_eq!(result, false);
    }

    #[test]
    fn hit_should_return_true_when_ray_is_along_face() {
        let b_box = BoundingBox::new(Vec3d::new(1.0, 1.0, 1.0), Vec3d::new(3.0, 3.0, 3.0));
        let ray = Ray::new(Vec3d::new(1.0, 1.5, 0.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = b_box.hit(&ray, 0.0, f64::MAX);

        assert_eq!(result, true);
    }

    #[test]
    fn hit_should_return_true_when_ray_starts_inside_box() {
        let b_box = BoundingBox::new(Vec3d::new(1.0, 1.0, 1.0), Vec3d::new(3.0, 3.0, 3.0));
        let ray = Ray::new(Vec3d::new(2.0, 2.0, 2.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = b_box.hit(&ray, 0.0, f64::MAX);

        assert_eq!(result, true);
    }
}
