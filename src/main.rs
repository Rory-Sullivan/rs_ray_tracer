use std::{fs::File, io::Write, time::Instant};

use indicatif::ProgressBar;
use rs_ray_tracer::{
    colour::RGB,
    hittable::{Hittable, HittableList},
    material::{Diffuse, Metal},
    utilities::random,
    Camera, Ray, Sphere, Vec3d,
};

fn main() {
    // Image
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = 225;
    const ASPECT_RATIO: f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);
    const NUM_SAMPLES: usize = 100;
    const MAX_DEPTH: usize = 50;

    const OUTPUT_FILE_NAME: &str = "result.ppm";

    // Camera
    let camera = Camera::new(ASPECT_RATIO);

    // Scene
    let material_ground = Diffuse::new(RGB(0.8, 0.8, 0.0));
    let material_center = Diffuse::new(RGB(0.7, 0.3, 0.3));
    let material_left = Metal::new(RGB(0.8, 0.8, 0.8), 0.0);
    let material_right = Metal::new(RGB(0.8, 0.6, 0.2), 1.0);

    let ground_sphere = Sphere::new(Vec3d::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    let center_sphere = Sphere::new(Vec3d::new(0.0, 0.0, -1.0), 0.5, &material_center);
    let left_sphere = Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    let right_sphere = Sphere::new(Vec3d::new(1.0, 0.0, -1.0), 0.5, &material_right);

    let mut scene = HittableList::new();
    scene.add(Box::new(ground_sphere));
    scene.add(Box::new(center_sphere));
    scene.add(Box::new(left_sphere));
    scene.add(Box::new(right_sphere));

    // Render
    println!("Starting");
    let start_instant = Instant::now();
    let progress_increments = 10;
    let progress_bar = ProgressBar::new(IMAGE_HEIGHT as u64);

    let mut image_string: String = format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").to_string();

    // Bottom -> top
    for j in (0..IMAGE_HEIGHT).rev() {
        // Print dot for progress
        if j < IMAGE_HEIGHT && j % progress_increments == 0 {
            progress_bar.inc(progress_increments as u64);
        }

        // Left -> right
        for i in 0..IMAGE_WIDTH {
            let mut colour = RGB(0.0, 0.0, 0.0);
            for _ in 0..NUM_SAMPLES {
                let u = ((i as f64) + random()) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random()) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = camera.get_ray(u, v);

                colour = colour + ray_colour(&ray, &scene, MAX_DEPTH)
            }
            image_string.push_str(&colour.write_colour(NUM_SAMPLES));
        }
    }
    progress_bar.finish();
    print!("\n");

    let mut output_file = File::create(OUTPUT_FILE_NAME).unwrap();
    output_file.write_all(image_string.as_bytes()).unwrap();

    let duration = start_instant.elapsed();
    println!("DONE, time taken: {duration:?}");
}

fn ray_colour(ray: &Ray, scene: &HittableList, max_depth: usize) -> RGB {
    if max_depth <= 0 {
        return RGB(0.0, 0.0, 0.0);
    }

    let hit = scene.hit(&ray, 0.001, f64::MAX);
    match hit {
        Some(hr) => match hr.material.scatter(ray, &hr) {
            Some((ray_out, hit_colour)) => hit_colour * ray_colour(&ray_out, scene, max_depth - 1),
            None => RGB(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * RGB(1.0, 1.0, 1.0) + t * RGB(0.5, 0.7, 1.0)
        }
    }
}
