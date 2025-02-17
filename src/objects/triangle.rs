use crate::{
    bvh::bounding_box::BoundingBox,
    hittable::{hit_record::HitRecord, hittable::Hittable},
    materials::material::Material,
    ray::Ray,
    utilities::{max, min},
    vec3d::{Point3d, Vec3d},
};

/// A triangle object that stores the three vertices of the triangle.
#[derive(Debug, Clone, Copy)]
pub struct Triangle<TMaterial>
where
    TMaterial: Material + Clone + Sync,
{
    a: Point3d,
    b: Point3d,
    c: Point3d,
    e1: Vec3d,
    e2: Vec3d,
    normal: Vec3d,
    material: TMaterial,
}

impl<TMaterial> Triangle<TMaterial>
where
    TMaterial: Material + Clone + Sync,
{
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d, material: TMaterial) -> Triangle<TMaterial> {
        let e1 = b - a;
        let e2 = c - a;
        let normal = e1.cross(&e2).unit_vector();

        Self {
            a,
            b,
            c,
            e1,
            e2,
            normal,
            material,
        }
    }
}

impl<TMaterial> Hittable for Triangle<TMaterial>
where
    TMaterial: Material + Clone + Sync,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match moller_trumbore_triangle_intersection(ray, self, t_min, t_max) {
            Some((t, u, v, intersection_point, outward_normal)) => {
                let (front_face, normal) = HitRecord::get_face_normal(ray, outward_normal);

                Some(HitRecord::new(
                    intersection_point,
                    normal,
                    &self.material,
                    t,
                    u,
                    v,
                    front_face,
                ))
            }
            None => None,
        }
    }

    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
    ) -> Option<crate::bvh::bounding_box::BoundingBox> {
        let mut x0 = min(self.a.x, self.b.x);
        x0 = min(x0, self.c.x);
        let mut x1 = max(self.a.x, self.b.x);
        x1 = max(x1, self.c.x);
        let len = x1 - x0;
        // If the edge of the box is very narrow pad it slightly
        if len > -f64::EPSILON && len < f64::EPSILON {
            x0 -= 0.001;
            x1 += 0.001;
        }

        let mut y0 = min(self.a.y, self.b.y);
        y0 = min(y0, self.c.y);
        let mut y1 = max(self.a.y, self.b.y);
        y1 = max(y1, self.c.y);
        let len = y1 - y0;
        // If the edge of the box is very narrow pad it slightly
        if len > -f64::EPSILON && len < f64::EPSILON {
            y0 -= 0.001;
            y1 += 0.001;
        }

        let mut z0 = min(self.a.z, self.b.z);
        z0 = min(z0, self.c.z);
        let mut z1 = max(self.a.z, self.b.z);
        z1 = max(z1, self.c.z);
        let len = z1 - z0;
        // If the edge of the box is very narrow pad it slightly
        if len > -f64::EPSILON && len < f64::EPSILON {
            z0 -= 0.001;
            z1 += 0.001;
        }

        Some(BoundingBox::new(
            Point3d::new(x0, y0, z0),
            Point3d::new(x1, y1, z1),
        ))
    }
}

/// An implementation of the Moller Trumbore triangle intersection algorithm.
/// Returns None if the ray does not intersect the triangle, otherwise returns
/// t, u, v, the point of intersection, and normal to the plane.
///
/// Read more about it here
/// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
fn moller_trumbore_triangle_intersection<TMaterial>(
    ray: &Ray,
    triangle: &Triangle<TMaterial>,
    t_min: f64,
    t_max: f64,
) -> Option<(f64, f64, f64, Point3d, Vec3d)>
where
    TMaterial: Material + Clone + Sync,
{
    let ray_cross_e2 = ray.direction.cross(&triangle.e2);
    let det = triangle.e1.dot(&ray_cross_e2);
    if det > -f64::EPSILON && det < f64::EPSILON {
        // The ray is parallel to the triangle and so it will not intersect
        return None;
    }

    let inv_det = 1.0 / det;
    let s = ray.origin - triangle.a;
    let u = inv_det * s.dot(&ray_cross_e2);
    if u < 0.0 || u > 1.0 {
        // Intersection of plane occurs outside triangle
        return None;
    }

    let s_cross_e1 = s.cross(&triangle.e1);
    let v = inv_det * ray.direction.dot(&s_cross_e1);
    if v < 0.0 || u + v > 1.0 {
        // Intersection of plane occurs outside triangle
        return None;
    }

    // Now we have a hit on the the line of the ray
    let t = inv_det * triangle.e2.dot(&s_cross_e1);
    if t < t_min || t > t_max {
        // There is a line intersection but not a ray intersection
        return None;
    }

    let intersection_point = ray.at(t);
    return Some((t, u, v, intersection_point, triangle.normal));
}

#[cfg(test)]
mod moller_trumbore_triangle_intersection_tests {
    use crate::{colour::RGB, materials::diffuse::Diffuse};

    use super::*;

    #[test]
    fn hit_should_return_none_when_ray_misses() {
        let material: Diffuse = Diffuse::new(RGB(0.0, 0.0, 0.0));
        let tri = Triangle::new(
            Vec3d::new(-1.0, -1.0, 0.0),
            Vec3d::new(1.0, -1.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            material,
        );
        let ray = Ray::new(Vec3d::new(2.0, 2.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = moller_trumbore_triangle_intersection(&ray, &tri, 0.0, f64::MAX);

        assert!(result.is_none());
    }

    #[test]
    fn hit_should_return_none_when_ray_parallel_to_triangle() {
        let material: Diffuse = Diffuse::new(RGB(0.0, 0.0, 0.0));
        let tri = Triangle::new(
            Vec3d::new(-1.0, -1.0, 0.0),
            Vec3d::new(1.0, -1.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            material,
        );
        let ray = Ray::new(Vec3d::new(-5.0, 0.0, 0.0), Vec3d::new(1.0, 0.0, 0.0), 0.0);

        let result = moller_trumbore_triangle_intersection(&ray, &tri, 0.0, f64::MAX);

        assert!(result.is_none());
    }

    #[test]
    fn hit_should_return_none_when_tri_behind_ray() {
        let material: Diffuse = Diffuse::new(RGB(0.0, 0.0, 0.0));
        let tri = Triangle::new(
            Vec3d::new(-1.0, -1.0, 0.0),
            Vec3d::new(1.0, -1.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            material,
        );
        let ray = Ray::new(Vec3d::new(0.0, 0.0, 5.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = moller_trumbore_triangle_intersection(&ray, &tri, 0.0, f64::MAX);

        assert!(result.is_none());
    }

    #[test]
    fn hit_should_return_values_when_ray_hits_middle() {
        let material: Diffuse = Diffuse::new(RGB(0.0, 0.0, 0.0));
        let tri = Triangle::new(
            Vec3d::new(-1.0, -1.0, 0.0),
            Vec3d::new(1.0, -1.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            material,
        );
        let ray = Ray::new(Vec3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = moller_trumbore_triangle_intersection(&ray, &tri, 0.0, f64::MAX);

        assert!(result.is_some());

        let (t, u, v, intersection_point, outward_normal) = result.unwrap();
        assert_eq!(t, 5.0);
        assert_eq!(u, 0.25);
        assert_eq!(v, 0.5);
        assert_eq!(intersection_point, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(outward_normal, Point3d::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn hit_should_return_values_when_ray_hits_vertex() {
        let material: Diffuse = Diffuse::new(RGB(0.0, 0.0, 0.0));
        let tri = Triangle::new(
            Vec3d::new(-1.0, -1.0, 0.0),
            Vec3d::new(1.0, -1.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            material,
        );
        let ray = Ray::new(Vec3d::new(0.0, 1.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), 0.0);

        let result = moller_trumbore_triangle_intersection(&ray, &tri, 0.0, f64::MAX);

        assert!(result.is_some());

        let (t, u, v, intersection_point, outward_normal) = result.unwrap();
        assert_eq!(t, 5.0);
        assert_eq!(u, 0.0);
        assert_eq!(v, 1.0);
        assert_eq!(intersection_point, Point3d::new(0.0, 1.0, 0.0));
        assert_eq!(outward_normal, Point3d::new(0.0, 0.0, 1.0));
    }
}
