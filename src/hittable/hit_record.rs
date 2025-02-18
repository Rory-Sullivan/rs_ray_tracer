use crate::{
    materials::Material,
    ray::Ray,
    vec3d::{Point3d, Vec3d},
};

pub struct HitRecord<'a> {
    pub point: Point3d,
    pub normal: Vec3d,
    pub material: &'a dyn Material,
    pub t: f64,
    pub u: f64, // value in [0, 1) representing the angle around the y-axis from x=-1 on unit sphere where hit occurs
    pub v: f64, // value in [0, 1) representing the angle from y=-1 to y=+1 on unit sphere where hit occurs
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3d,
        normal: Vec3d,
        material: &'a dyn Material,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> Self {
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
