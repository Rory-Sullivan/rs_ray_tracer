use super::bounding_box::BoundingBox;
use std::cmp::Ordering;

use crate::{
    hittable::{hit_record::HitRecord, hittable::Hittable, hittable_list_dyn::HittableListDyn},
    ray::Ray,
    utilities::surrounding_box,
};

/// Bounding Volume Hierarchy. Used to store hittable objects in a tree like
/// structure to make finding a hit more efficient.
#[derive(Clone)]
pub struct Bvh<'a> {
    left: Box<dyn Hittable + 'a>,
    right: Box<dyn Hittable + 'a>,
    bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Copy)]
pub struct BvhMetrics {
    min_depth: usize,
    max_depth: usize,
    average_depth: f32,
}

impl<'a> Bvh<'a> {
    pub fn new(
        left: Box<dyn Hittable + 'a>,
        right: Box<dyn Hittable + 'a>,
        bounding_box: BoundingBox,
    ) -> Self {
        Bvh {
            left,
            right,
            bounding_box,
        }
    }

    /// Builds a BVH from scene data.
    pub fn build(scene: HittableListDyn<'a>, time0: f64, time1: f64) -> (Self, BvhMetrics) {
        Self::build_internal(scene, time0, time1, 0)
    }

    fn build_internal(
        scene: HittableListDyn<'a>,
        time0: f64,
        time1: f64,
        mut current_depth: usize,
    ) -> (Self, BvhMetrics) {
        current_depth += 1;

        // Pick the longest axis along which to split the objects
        let axis = scene.bounding_box(time0, time1).unwrap().longest_axis();
        let compare_fn = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Axis is out of range; axis: {axis}"),
        };

        // Order and split list of objects based on axis
        let mut objects = scene.items;
        let num_objects = objects.len();
        let (
            left,
            left_min_depth,
            left_max_depth,
            left_average_depth,
            right,
            right_min_depth,
            right_max_depth,
            right_average_depth,
        ): (
            Box<dyn Hittable + 'a>,
            usize,
            usize,
            f32,
            Box<dyn Hittable + 'a>,
            usize,
            usize,
            f32,
        ) = match num_objects {
            0 => panic!("No objects"),
            1 => {
                let left = objects[0].clone();
                let right = objects[0].clone();
                (
                    left,
                    current_depth,
                    current_depth,
                    current_depth as f32,
                    right,
                    current_depth,
                    current_depth,
                    current_depth as f32,
                )
            }
            2 => match compare_fn(&objects[0], &objects[1]) {
                Ordering::Less | Ordering::Equal => {
                    let left = objects[0].clone();
                    let right = objects[1].clone();
                    (
                        left,
                        current_depth,
                        current_depth,
                        current_depth as f32,
                        right,
                        current_depth,
                        current_depth,
                        current_depth as f32,
                    )
                }
                Ordering::Greater => {
                    let left = objects[1].clone();
                    let right = objects[0].clone();
                    (
                        left,
                        current_depth,
                        current_depth,
                        current_depth as f32,
                        right,
                        current_depth,
                        current_depth,
                        current_depth as f32,
                    )
                }
            },
            _ => {
                objects.sort_by(compare_fn);

                // Recursively call build function with split parts
                let mid = num_objects / 2;
                let (half0, half1) = objects.split_at_mut(mid);

                let hit_list0 = HittableListDyn::build(time0, time1, half0.to_vec());
                let (left, left_metrics) =
                    Self::build_internal(hit_list0, time0, time1, current_depth);

                let hit_list1 = HittableListDyn::build(time0, time1, half1.to_vec());
                let (right, right_metrics) =
                    Self::build_internal(hit_list1, time0, time1, current_depth);

                (
                    Box::new(left),
                    left_metrics.min_depth,
                    left_metrics.max_depth,
                    left_metrics.average_depth,
                    Box::new(right),
                    right_metrics.min_depth,
                    right_metrics.max_depth,
                    right_metrics.average_depth,
                )
            }
        };

        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();
        let bounding_box = surrounding_box(box_left, box_right);

        let min_depth = left_min_depth.min(right_min_depth);
        let max_depth = left_max_depth.max(right_max_depth);
        let average_depth = (left_average_depth + right_average_depth) / 2.0;

        let metrics = BvhMetrics {
            min_depth,
            max_depth,
            average_depth,
        };

        (Bvh::new(left, right, bounding_box), metrics)
    }
}

impl<'a> Hittable for Bvh<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Check if we hit the bounding box
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        // If we hit the bounding box check if we hit left or right bounding box
        // and return the closer of the two
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        match self.left.hit(ray, t_min, closest_so_far) {
            Some(hr) => {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
            None => {}
        }
        let hit_right = self.right.hit(ray, t_min, closest_so_far);
        match hit_right {
            Some(_) => {
                hit_record = hit_right;
            }
            None => {}
        }

        hit_record
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        Some(self.bounding_box)
    }
}

fn box_compare<'a>(
    a: &Box<dyn Hittable + 'a>,
    b: &Box<dyn Hittable + 'a>,
    axis: usize,
) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    box_a
        .min
        .get_axis(axis)
        .total_cmp(&box_b.min.get_axis(axis))
}

fn box_x_compare<'a>(a: &Box<dyn Hittable + 'a>, b: &Box<dyn Hittable + 'a>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare<'a>(a: &Box<dyn Hittable + 'a>, b: &Box<dyn Hittable + 'a>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare<'a>(a: &Box<dyn Hittable + 'a>, b: &Box<dyn Hittable + 'a>) -> Ordering {
    box_compare(a, b, 2)
}
