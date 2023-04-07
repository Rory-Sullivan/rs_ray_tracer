use std::{f64::RADIX, fs::File, io::Write, time::Instant};

use rs_ray_tracer::{colour::RGB, Point3d, Ray, Vec3d};

fn main() {
    // Image
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = 225;
    const ASPECT_RATIO: f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);

    const OUTPUT_FILE_NAME: &str = "result.ppm";

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3d::new(0.0, 0.0, 0.0);
    let horizontal = Vec3d::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3d::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3d::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    println!("Starting");
    let start_instant = Instant::now();

    let mut image_string: String = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").to_string();

    // Bottom -> top
    for j in (0..IMAGE_HEIGHT).rev() {
        // Print dot for progress
        if j % 10 == 0 {
            print!(".");
        }

        // Left -> right
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);

            let colour = ray_colour(&ray);
            image_string.push_str(&colour.write_colour());
        }
    }
    print!("\n");

    let mut output_file = File::create(OUTPUT_FILE_NAME).unwrap();
    output_file.write_all(image_string.as_bytes()).unwrap();

    let duration = start_instant.elapsed();
    println!("DONE, time taken: {duration:?}");
}

fn ray_colour(ray: &Ray) -> RGB {
    if hit_sphere(&Vec3d::new(0.0, 0.0, -1.0), 0.5, ray) {
        return RGB(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * RGB(1.0, 1.0, 1.0) + t * RGB(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3d, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
