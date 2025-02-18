use crate::{
    materials::Material,
    ray::Ray,
    vec3d::{Point3d, Vec3d},
};

/// Stores data related to a ray intersecting with a surface.
#[non_exhaustive]
pub struct HitRecord<'a> {
    /// The point where the ray hits the surface.
    pub point: Point3d,
    /// The unit normal of the surface at the hit point.
    pub normal: Vec3d,
    /// Reference to the material of the surface.
    pub material: &'a dyn Material,
    /// The distance along the ray that the hit occurs, always positive.
    pub t: f64,
    /// Value in [0, 1) representing the angle around the y-axis from x=-1 on
    /// unit sphere where hit occurs.
    pub u: f64,
    /// Value in [0, 1) representing the angle from y=-1 to y=+1 on unit sphere
    /// where hit occurs.
    pub v: f64,
    /// Whether or not this is the external face of the surface, this is useful
    /// to know for dielectrics.
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    /// Creates a new instance of `HitRecord`.
    ///
    /// * `point`: The point where the ray hits the surface.
    /// * `normal`: The unit normal of the surface at the hit point.
    /// * `material`: Reference to the material of the surface.
    /// * `t`: The distance along the ray that the hit occurs.
    /// * `u`: Value in [0, 1) representing the angle around the y-axis from
    ///   x=-1 on unit sphere where hit occurs.
    /// * `v`: Value in [0, 1) representing the angle from y=-1 to y=+1 on unit
    ///   sphere where hit occurs.
    /// * `front_face`: Whether or not this is the external face of the surface,
    ///   this is useful to know for dielectrics.
    pub fn new(
        point: Point3d,
        normal: Vec3d,
        material: &'a dyn Material,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> Self {
        #[cfg(debug_assertions)]
        {
            const DELTA: f64 = 0.1e-5;
            let normal_len = normal.len();
            assert!(
                normal_len >= 1.0 - DELTA && normal_len <= 1.0 + DELTA,
                "Length of normal is not 1; length: {normal_len}"
            );
            assert!(u >= 0.0 && u < 1.0, "u is outside range [0, 1); u: {u}");
            assert!(v >= 0.0 && v < 1.0, "v is outside range [0, 1); u: {v}");
        }

        Self {
            point,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn get_face_normal(ray: &Ray, outward_normal: Vec3d) -> (bool, Vec3d) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        (front_face, normal)
    }
}
