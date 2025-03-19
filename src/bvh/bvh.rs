use super::bounding_box::BoundingBox;
use std::cmp::Ordering;

use crate::{
    hittable::{hit_record::HitRecord, hittable::Hittable},
    ray::Ray,
    utilities::surrounding_box_option,
};

pub type BvhNode = Option<Box<dyn Hittable>>;

/// Bounding Volume Hierarchy. Used to store hittable objects in a tree like
/// structure to make finding a hit more efficient.
#[derive(Debug, Clone)]
pub struct Bvh {
    left: BvhNode,
    right: BvhNode,
    bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Copy)]
pub struct BvhMetrics {
    min_depth: usize,
    max_depth: usize,
    average_depth: f32,
}

impl Bvh {
    /// Builds a BVH from scene data.
    pub fn build(time0: f64, time1: f64, items: &mut [Box<dyn Hittable>]) -> (Self, BvhMetrics) {
        Self::build_internal(time0, time1, items, 0)
    }

    fn build_internal(
        time0: f64,
        time1: f64,
        items: &mut [Box<dyn Hittable>],
        mut current_depth: usize,
    ) -> (Self, BvhMetrics) {
        current_depth += 1;

        // Pick the longest axis along which to split the objects
        let items_bounding_box: Option<BoundingBox> = items.iter().fold(None, |bb, item| {
            surrounding_box_option(bb, item.bounding_box(time0, time1))
        });
        let axis = items_bounding_box
            .expect("Items to have a valid bounding box")
            .longest_axis();
        let compare_fn = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Axis is out of range; axis: {axis}"),
        };

        // Order and split list of objects based on axis
        let num_objects = items.len();
        #[allow(clippy::type_complexity)]
        let (
            left,
            left_min_depth,
            left_max_depth,
            left_average_depth,
            right,
            right_min_depth,
            right_max_depth,
            right_average_depth,
        ): (BvhNode, usize, usize, f32, BvhNode, usize, usize, f32) = match num_objects {
            0 => panic!("No objects"),
            1 => {
                let left = items[0].clone();
                (
                    Some(left),
                    current_depth,
                    current_depth,
                    current_depth as f32,
                    None,
                    current_depth,
                    current_depth,
                    current_depth as f32,
                )
            }
            2 => match compare_fn(&items[0], &items[1]) {
                Ordering::Less | Ordering::Equal => {
                    let left = items[0].clone();
                    let right = items[1].clone();
                    (
                        Some(left),
                        current_depth,
                        current_depth,
                        current_depth as f32,
                        Some(right),
                        current_depth,
                        current_depth,
                        current_depth as f32,
                    )
                }
                Ordering::Greater => {
                    let left = items[1].clone();
                    let right = items[0].clone();
                    (
                        Some(left),
                        current_depth,
                        current_depth,
                        current_depth as f32,
                        Some(right),
                        current_depth,
                        current_depth,
                        current_depth as f32,
                    )
                }
            },
            _ => {
                items.sort_by(compare_fn);

                // Recursively call build function with split parts
                let mid = num_objects / 2;
                let (half0, half1) = items.split_at_mut(mid);

                let (left, left_metrics) = Self::build_internal(time0, time1, half0, current_depth);

                let (right, right_metrics) =
                    Self::build_internal(time0, time1, half1, current_depth);

                (
                    Some(Box::new(left)),
                    left_metrics.min_depth,
                    left_metrics.max_depth,
                    left_metrics.average_depth,
                    Some(Box::new(right)),
                    right_metrics.min_depth,
                    right_metrics.max_depth,
                    right_metrics.average_depth,
                )
            }
        };

        let bounding_box = surrounding_box_option(
            left.as_ref().and_then(|x| x.bounding_box(time0, time1)),
            right.as_ref().and_then(|x| x.bounding_box(time0, time1)),
        )
        .expect("surrounding box to be valid");

        let min_depth = left_min_depth.min(right_min_depth);
        let max_depth = left_max_depth.max(right_max_depth);
        let average_depth = (left_average_depth + right_average_depth) / 2.0;

        let metrics = BvhMetrics {
            min_depth,
            max_depth,
            average_depth,
        };

        (
            Bvh {
                left,
                right,
                bounding_box,
            },
            metrics,
        )
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Check if we hit the bounding box
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        // If we hit the bounding box check if we hit left or right bounding box
        // and return the closer of the two
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        if let Some(hr) = self
            .left
            .as_ref()
            .and_then(|x| x.hit(ray, t_min, closest_so_far))
        {
            closest_so_far = hr.t;
            hit_record = Some(hr);
        }
        let hit_right = self
            .right
            .as_ref()
            .and_then(|x| x.hit(ray, t_min, closest_so_far));
        if hit_right.is_some() {
            hit_record = hit_right;
        }

        hit_record
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        Some(self.bounding_box)
    }
}

#[allow(clippy::borrowed_box)]
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

#[allow(clippy::borrowed_box)]
fn box_x_compare<'a>(a: &Box<dyn Hittable + 'a>, b: &Box<dyn Hittable + 'a>) -> Ordering {
    box_compare(a, b, 0)
}

#[allow(clippy::borrowed_box)]
fn box_y_compare<'a>(a: &Box<dyn Hittable + 'a>, b: &Box<dyn Hittable + 'a>) -> Ordering {
    box_compare(a, b, 1)
}

#[allow(clippy::borrowed_box)]
fn box_z_compare<'a>(a: &Box<dyn Hittable + 'a>, b: &Box<dyn Hittable + 'a>) -> Ordering {
    box_compare(a, b, 2)
}
