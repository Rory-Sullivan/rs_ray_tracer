use rand::Rng;
use std::f64::consts::PI;

use crate::Vec3d;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random number in [0, 1)
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// Returns a random number in [min, max)
pub fn random_rng(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

/// Returns a random vector where x, y, and z are all in [0, 1)
pub fn random_vec() -> Vec3d {
    Vec3d::new(random(), random(), random())
}

/// Returns a random vector where x, y, and z are all in [min, max)
pub fn random_vec_rng(min: f64, max: f64) -> Vec3d {
    Vec3d::new(
        random_rng(min, max),
        random_rng(min, max),
        random_rng(min, max),
    )
}

/// Returns a random vector inside the unit sphere
pub fn random_vec_in_unit_sphere() -> Vec3d {
    loop {
        let v = random_vec_rng(-1.0, 1.0);
        if v.len_squared() < 1.0 {
            return v;
        }
    }
}

/// Returns a random vector on the unit sphere
pub fn random_unit_vec() -> Vec3d {
    random_vec_in_unit_sphere().unit_vector()
}

pub fn random_vec_in_hemisphere(normal: &Vec3d) -> Vec3d {
    let r = random_vec_in_unit_sphere();
    if r.dot(normal) > 0.0 {
        return r;
    }
    -1.0 * r
}

pub fn reflect_vec(vec_in: &Vec3d, normal: &Vec3d) -> Vec3d {
    *vec_in - 2.0 * vec_in.dot(normal) * *normal
}

pub fn refract_vec(vec_in: &Vec3d, normal: &Vec3d, refraction_index: f64) -> Vec3d {
    let cos_theta = f64::min(-vec_in.dot(normal), 1.0);
    let vec_out_perpendicular = refraction_index * (*vec_in + cos_theta * *normal);
    let vec_out_parallel =
        -1.0 * (1.0 - vec_out_perpendicular.len_squared()).abs().sqrt() * *normal;
    vec_out_perpendicular + vec_out_parallel
}

pub fn clamp(num: f64, min: f64, max: f64) -> f64 {
    if num < min {
        return min;
    }
    if num > max {
        return max;
    }
    num
}
