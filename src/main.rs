use std::{fs::File, io::Write, time::Instant};

use indicatif::ProgressBar;
use rs_ray_tracer::{
    colour::RGB,
    hittable::{Hittable, HittableList},
    utilities::{random, random_vec_in_hemisphere},
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
    let mut scene = HittableList::<Sphere>::new();
    scene.add(Sphere::new(Vec3d::new(0.0, 0.0, -1.0), 0.5));
    scene.add(Sphere::new(Vec3d::new(0.0, -100.5, -1.0), 100.0));

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

                let new_colour = ray_colour(&ray, &scene, MAX_DEPTH);
                colour = RGB(
                    colour.0 + new_colour.0,
                    colour.1 + new_colour.1,
                    colour.2 + new_colour.2,
                );
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

fn ray_colour<TScene>(ray: &Ray, scene: &TScene, max_depth: usize) -> RGB
where
    TScene: Hittable,
{
    if max_depth <= 0 {
        return RGB(0.0, 0.0, 0.0);
    }

    let hit = scene.hit(&ray, 0.001, f64::MAX);
    match hit {
        Some(hr) => {
            let target = hr.point + hr.normal + random_vec_in_hemisphere(&hr.normal);
            let target_ray = Ray::new(hr.point, target - hr.point);
            0.5 * ray_colour(&target_ray, scene, max_depth - 1)
        }
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * RGB(1.0, 1.0, 1.0) + t * RGB(0.5, 0.7, 1.0)
        }
    }
}
