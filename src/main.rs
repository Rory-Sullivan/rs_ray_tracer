use rayon::prelude::*;
use std::time::Instant;

use indicatif::ProgressBar;
use rs_ray_tracer::{
    colour::RGB,
    hittable::{Hittable, HittableList},
    material::{Dielectric, Diffuse, Metal},
    utilities::{random, random_rgb, random_rng, save_as_png, save_as_ppm},
    Camera, Point3d, Ray, Sphere, Vec3d,
};

fn main() {
    // High res image
    // const IMAGE_WIDTH: usize = 1200;
    // const IMAGE_HEIGHT: usize = 800;
    // const NUM_SAMPLES: usize = 500;
    // const MAX_DEPTH: usize = 50;

    // Low res image
    const IMAGE_WIDTH: usize = 600;
    const IMAGE_HEIGHT: usize = 400;
    const NUM_SAMPLES: usize = 100;
    const MAX_DEPTH: usize = 50;

    const ASPECT_RATIO: f64 = (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);

    const FOV: f64 = 20.0; // degrees
    let look_from: Point3d = Point3d::new(13.0, 2.0, 3.0);
    let look_at: Point3d = Point3d::new(0.0, 0.0, 0.0);
    let view_up: Vec3d = Vec3d::new(0.0, 1.0, 0.0);
    const APERTURE: f64 = 0.1;
    let focus_dist: f64 = 10.0;

    const OUTPUT_FILE_NAME_PPM: &str = "result.ppm";
    const OUTPUT_FILE_NAME_PNG: &str = "result.png";

    // Camera
    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        FOV,
        ASPECT_RATIO,
        APERTURE,
        focus_dist,
    );
    // Scene

    // let scene = generate_basic_scene();
    let scene = generate_random_complex_scene();

    // Render
    println!("Starting");
    let start_instant = Instant::now();
    let progress_increments = 10;
    let progress_bar = ProgressBar::new(IMAGE_HEIGHT as u64);

    let mut pixels: Vec<(usize, usize)> = Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH);
    // Bottom -> top
    for j in (0..IMAGE_HEIGHT).rev() {
        // Left -> right
        for i in 0..IMAGE_WIDTH {
            pixels.push((i, j))
        }
    }

    let image: Vec<RGB> = pixels
        .par_iter()
        .map(|pixel| {
            let mut colour = RGB(0.0, 0.0, 0.0);
            for _ in 0..NUM_SAMPLES {
                let u = ((pixel.0 as f64) + random()) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((pixel.1 as f64) + random()) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = camera.get_ray(u, v);

                colour = colour + ray_colour(&ray, &scene, MAX_DEPTH)
            }

            if (pixel.1 < IMAGE_HEIGHT) && (pixel.1 % progress_increments == 0) && (pixel.0 == 0) {
                progress_bar.inc(progress_increments as u64);
            }
            colour
        })
        .collect();

    progress_bar.finish();
    print!("\n");

    save_as_png(
        OUTPUT_FILE_NAME_PNG,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        &image,
        NUM_SAMPLES,
    );

    save_as_ppm(
        OUTPUT_FILE_NAME_PPM,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        &image,
        NUM_SAMPLES,
    );

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

fn generate_basic_scene<'a>() -> HittableList<'a> {
    // Basic scene
    let material_ground = Diffuse::new(RGB(0.8, 0.8, 0.0));
    let material_centre = Diffuse::new(RGB(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(RGB(0.8, 0.6, 0.2), 0.0);

    let ground = Sphere::new(Vec3d::new(0.0, -100.5, 1.0), 100.0, material_ground);
    let centre_sphere = Sphere::new(Vec3d::new(0.0, 0.0, -1.0), 0.5, material_centre);
    let left_sphere = Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let left_inner_sphere = Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), -0.45, material_left);
    let right_sphere = Sphere::new(Vec3d::new(1.0, 0.0, -1.0), 0.5, material_right);

    let mut scene = HittableList::new();
    scene.add(Box::new(ground));
    scene.add(Box::new(centre_sphere));
    scene.add(Box::new(left_sphere));
    scene.add(Box::new(left_inner_sphere));
    scene.add(Box::new(right_sphere));

    scene
}

fn generate_random_complex_scene<'a>() -> HittableList<'a> {
    let mut scene = HittableList::new();
    let material_ground = Diffuse::new(RGB(0.5, 0.5, 0.5));
    let ground = Sphere::new(Point3d::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    scene.add(Box::new(ground));

    // Add three large spheres
    let material1 = Dielectric::new(1.5);
    scene.add(Box::new(Sphere::new(
        Point3d::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Diffuse::new(RGB(0.4, 0.2, 0.1));
    scene.add(Box::new(Sphere::new(
        Point3d::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::new(RGB(0.7, 0.6, 0.5), 0.0);
    scene.add(Box::new(Sphere::new(
        Point3d::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Add several random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3d::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3d::new(4.0, 0.2, 0.0)).len() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse
                        let albedo = random_rgb() * random_rgb();
                        let sphere_material = Diffuse::new(albedo);
                        scene.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    x if x < 0.95 => {
                        // Metal
                        let albedo = random_rgb();
                        let fuzz = random_rng(0.0, 0.5);
                        let sphere_material = Metal::new(albedo, fuzz);
                        scene.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // Glass
                        let sphere_material = Dielectric::new(1.5);
                        scene.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                };
            }
        }
    }

    scene
}
