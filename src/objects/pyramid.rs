use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{HitRecord, Hittable, HittableList},
    materials::material::Material,
    ray::Ray,
    vec3d::Point3d,
};

use super::triangle::Triangle;

/// A triangle object that stores the three vertices of the triangle.
#[derive(Clone)]
pub struct Pyramid<'a> {
    sides: HittableList<'a>,
    bounding_box: Option<BoundingBox>,
}

impl<'a> Pyramid<'a> {
    pub fn new(sides: HittableList<'a>, bounding_box: Option<BoundingBox>) -> Self {
        Self {
            sides,
            bounding_box,
        }
    }

    /// Builds a pyramid object
    ///
    /// - base_triangle: a tuple representing half the base of the pyramid, the
    ///   first value is taken as the external corner of the pyramid
    /// - height: height of the point above the base
    /// - material: the material of the pyramid
    pub fn build<TMaterial>(
        base_triangle: (Point3d, Point3d, Point3d),
        height: f64,
        material: TMaterial,
    ) -> Pyramid<'a>
    where
        TMaterial: Material + Clone + Sync + 'a,
    {
        let (b0, b1, b2, b3, p) = get_pyramid_vertices(base_triangle, height);

        let mut sides = HittableList::new(0.0, 0.0);
        sides.add(Box::new(Triangle::new(b0, b1, p, material.clone())));
        sides.add(Box::new(Triangle::new(b1, b2, p, material.clone())));
        sides.add(Box::new(Triangle::new(b2, b3, p, material.clone())));
        sides.add(Box::new(Triangle::new(b3, b0, p, material.clone())));
        sides.add(Box::new(Triangle::new(b0, b1, b3, material.clone())));
        sides.add(Box::new(Triangle::new(b2, b3, b1, material.clone())));

        let bounding_box = sides.bounding_box(0.0, 0.0);

        Self::new(sides, bounding_box)
    }
}

impl<'a> Hittable for Pyramid<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        self.bounding_box
    }
}

fn get_pyramid_vertices(
    base_triangle: (Point3d, Point3d, Point3d),
    height: f64,
) -> (Point3d, Point3d, Point3d, Point3d, Point3d) {
    let b0 = base_triangle.0;
    let b1 = base_triangle.1;
    let b3 = base_triangle.2;

    let b3_b1 = b1 - b3;
    let b_center = b3 + (0.5 * b3_b1);
    let b0_b_center = b_center - b0;
    let b2 = b0 + (2.0 * b0_b_center);

    let e1 = b1 - b0;
    let e2 = b3 - b0;
    let b_normal = e1.cross(&e2).unit_vector();

    let p = b_center + (height * b_normal);

    (b0, b1, b2, b3, p)
}

#[cfg(test)]
mod get_pyramid_vertices_tests {
    use super::*;

    #[test]
    fn should_return_correct_vertices_for_simple_case() {
        let tri_base = (
            Point3d::new(0.0, 0.0, 0.0),
            Point3d::new(0.0, 0.0, 10.0),
            Point3d::new(10.0, 0.0, 0.0),
        );
        let height = 10.0;

        let result = get_pyramid_vertices(tri_base, height);

        assert_eq!(result.0, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(result.1, Point3d::new(0.0, 0.0, 10.0));
        assert_eq!(result.2, Point3d::new(10.0, 0.0, 10.0));
        assert_eq!(result.3, Point3d::new(10.0, 0.0, 0.0));
        assert_eq!(result.4, Point3d::new(5.0, 10.0, 5.0));
    }
}
