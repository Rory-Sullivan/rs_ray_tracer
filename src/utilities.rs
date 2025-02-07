use image::{ImageBuffer, RgbImage};
use rand::Rng;
use std::{
    cmp::{max_by, min_by},
    f64::consts::PI,
    fs::File,
    io::Write,
};

use crate::{colour::RGB, BoundingBox, Vec3d};

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

/// Returns a random integer in [min, max)
pub fn random_rng_int(min: usize, max: usize) -> usize {
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

pub fn random_vec_in_unit_disc() -> Vec3d {
    loop {
        let v = Vec3d::new(random_rng(-1.0, 1.0), random_rng(-1.0, 1.0), 0.0);
        if v.len_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_rgb() -> RGB {
    RGB(random(), random(), random())
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

pub fn min(a: f64, b: f64) -> f64 {
    min_by(a, b, |a, b| a.partial_cmp(b).unwrap())
}

pub fn max(a: f64, b: f64) -> f64 {
    max_by(a, b, |a, b| a.partial_cmp(b).unwrap())
}

pub fn save_as_ppm(
    file_name: &str,
    image_width: usize,
    image_height: usize,
    image: &Vec<RGB>,
    num_samples: usize,
) {
    let mut image_string: String = format!("P3\n{image_width} {image_height}\n255\n").to_string();
    for colour in image {
        image_string.push_str(&colour.write_colour(num_samples));
    }

    let mut output_file = File::create(file_name).unwrap();
    output_file.write_all(image_string.as_bytes()).unwrap();
}

pub fn save_as_png(
    file_name: &str,
    image_width: usize,
    image_height: usize,
    image: &Vec<RGB>,
    num_samples: usize,
) {
    let mut image_buffer: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);
    for (x, y, colour) in image_buffer.enumerate_pixels_mut() {
        let i = (y as usize * image_width) + x as usize;
        let pixel = image[i];
        let (ir, ig, ib) = pixel.to_integers(num_samples);
        colour.0 = [ir as u8, ig as u8, ib as u8];
    }
    image_buffer.save(file_name).unwrap();
}

pub fn surrounding_box(box0: BoundingBox, box1: BoundingBox) -> BoundingBox {
    let min = Vec3d::new(
        min(box0.min.x, box1.min.x),
        min(box0.min.y, box1.min.y),
        min(box0.min.z, box1.min.z),
    );
    let max = Vec3d::new(
        max(box0.max.x, box1.max.x),
        max(box0.max.y, box1.max.y),
        max(box0.max.z, box1.max.z),
    );

    BoundingBox::new(min, max)
}

/// Given a point on the unit sphere returns the coordinates of that point in
/// the form (u, v) where;
///
/// - u is a value in [0, 1) representing the angle around the y-axis from x=-1
/// - v is a value in [0, 1) representing the angle from y=-1 to y=+1
pub fn get_sphere_uv(point: Vec3d) -> (f64, f64) {
    let theta = (-point.y).acos();
    let phi = (-point.z).atan2(point.x) + PI;

    (phi / (2.0 * PI), theta / PI)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_sphere_uv_tests {
        use super::*;

        // (1, 0, 0) yields (0.50, 0.50)
        #[test]
        fn test_1() {
            let result = get_sphere_uv(Vec3d::new(1.0, 0.0, 0.0));
            assert_eq!(result, (0.5, 0.5));
        }

        // (-1, 0, 0) yields (0.00, 0.50)
        #[test]
        fn test_2() {
            let result = get_sphere_uv(Vec3d::new(-1.0, 0.0, 0.0));
            assert_eq!(result, (0.0, 0.5));
        }

        // (0, 1, 0) yields (0.50, 1.00)
        #[test]
        fn test_3() {
            let result = get_sphere_uv(Vec3d::new(0.0, 1.0, 0.0));
            assert_eq!(result, (0.5, 1.0));
        }

        // (0, -1, 0) yields (0.50, 0.00)
        #[test]
        fn test_4() {
            let result = get_sphere_uv(Vec3d::new(0.0, -1.0, 0.0));
            assert_eq!(result, (0.5, 0.0));
        }

        // (0, 0, 1) yields (0.25, 0.50)
        #[test]
        fn test_5() {
            let result = get_sphere_uv(Vec3d::new(0.0, 0.0, 1.0));
            assert_eq!(result, (0.25, 0.5));
        }

        // (0, 0, -1) yields (0.75, 0.50)
        #[test]
        fn test_6() {
            let result = get_sphere_uv(Vec3d::new(0.0, 0.0, -1.0));
            assert_eq!(result, (0.75, 0.5));
        }
    }
}
