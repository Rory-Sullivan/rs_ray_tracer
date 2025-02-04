use std::{fs::create_dir_all, time::Instant, usize};

use indicatif::ProgressBar;
use rs_ray_tracer::{
    colour::RGB,
    hittable::HittableList,
    material::{Dielectric, Diffuse, Metal},
    render::render_scene,
    utilities::{random, random_rgb, random_rng, save_as_png},
    Bvh, Camera, MovingSphere, Point3d, Resolution, Sphere, Vec3d,
};

fn main() {
    const OUTPUT_FOLDER: &str = "results";
    const OUTPUT_FILE_NAME: &str = "result";

    // Resolution
    // Low res
    let resolution = Resolution::new(
        600, // Image width
        400, // Image height
        100, // Num samples
        50,  // Max depth
    );
    // Med res
    // let resolution = Resolution::new(
    //     1200, // Image width
    //     800,  // Image height
    //     500,  // Num samples
    //     50,   // Max depth
    // );
    // High res (screen res)
    // let resolution = Resolution::new(
    //     1920, // Image width
    //     1080, // Image height
    //     800,  // Num samples
    //     50,   // Max depth
    // );

    // Camera
    const FOV: f64 = 20.0; // degrees
    const APERTURE: f64 = 0.1;
    const TIME0: f64 = 0.0; // Start time
    const TIME1: f64 = 1.0; // End time
    let cameras = [
        Camera::new(
            Point3d::new(13.0, 2.0, 3.0),  // Look from
            Point3d::new(0.0, 0.0, 0.0),   // Look at
            Vec3d::new(0.0, 1.0, 0.0),     // View up
            FOV,                           // Vertical field of view
            resolution.get_aspect_ratio(), // Aspect ratio
            APERTURE,                      // Aperture
            10.0,                          // Focus distance
            TIME0,                         // Start time
            TIME1,                         // End time
        ),
        // Camera::new(
        //     Point3d::new(5.0, 5.0, 13.0),  // Look from
        //     Point3d::new(0.0, 0.0, 0.0),   // Look at
        //     Vec3d::new(0.0, 1.0, 0.0),     // View up
        //     FOV,                           // Vertical field of view
        //     resolution.get_aspect_ratio(), // Aspect ratio
        //     APERTURE,                      // Aperture
        //     13.3,                          // Focus distance
        // ),
        // Camera::new(
        //     Point3d::new(-6.0, 1.0, -10.0), // Look from
        //     Point3d::new(4.0, 0.0, 0.0),    // Look at
        //     Vec3d::new(0.0, 1.0, 0.0),      // View up
        //     FOV,                            // Vertical field of view
        //     resolution.get_aspect_ratio(),  // Aspect ratio
        //     APERTURE,                       // Aperture
        //     16.0,                           // Focus distance
        // ),
    ];

    let start_instant = Instant::now();

    // Scene
    // let scene = generate_basic_scene();
    // let scene = generate_random_complex_scene();
    let mut scene = generate_random_complex_scene_moving_spheres();
    let start_bvh_build_instant = Instant::now();
    let bvh = Bvh::build(scene.items.as_mut_slice(), TIME0, TIME1);
    print_time_taken("Done building BVH", start_bvh_build_instant);

    // Render
    let start_render_instant = Instant::now();
    let num_cameras = cameras.len();
    for (i, camera) in cameras.iter().enumerate() {
        println!("Rendering camera {0}/{1} ", i + 1, num_cameras);
        let progress_increments = 10;
        let progress_bar = ProgressBar::new(resolution.image_height as u64);

        let increment_progress_bar = |row_number: usize| {
            if (row_number < resolution.image_height) && (row_number % progress_increments == 0) {
                progress_bar.inc(progress_increments as u64);
            }
        };
        let image = render_scene(&camera, &bvh, &resolution, increment_progress_bar);

        progress_bar.finish();
        print!("\n");

        println!("Saving PNG");
        create_dir_all(OUTPUT_FOLDER).unwrap();
        let file_name_png = format!("{0}/{1}_{2}.png", OUTPUT_FOLDER, OUTPUT_FILE_NAME, i + 1);
        save_as_png(
            &file_name_png,
            resolution.image_width,
            resolution.image_height,
            &image,
            resolution.num_samples,
        );
    }

    print_time_taken("Done rendering", start_render_instant);
    print_time_taken("DONE", start_instant);
}

fn print_time_taken(message: &str, start_instant: Instant) {
    let duration_secs = start_instant.elapsed().as_secs();
    let duration_mins = duration_secs / 60;
    let remaining_secs = duration_secs % 60;
    println!("{message}, time taken: {duration_mins}m {remaining_secs}s ({duration_secs}s)");
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn generate_random_complex_scene_moving_spheres<'a>() -> HittableList<'a> {
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
            let center0 = Point3d::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());
            let center1 = center0 + Vec3d::new(0.0, random_rng(0.0, 0.5), 0.0);

            if (center0 - Point3d::new(4.0, 0.2, 0.0)).len() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse
                        let albedo = random_rgb() * random_rgb();
                        let sphere_material = Diffuse::new(albedo);
                        scene.add(Box::new(MovingSphere::new(
                            center0,
                            center1,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material,
                        )));
                    }
                    x if x < 0.95 => {
                        // Metal
                        let albedo = random_rgb();
                        let fuzz = random_rng(0.0, 0.5);
                        let sphere_material = Metal::new(albedo, fuzz);
                        scene.add(Box::new(MovingSphere::new(
                            center0,
                            center1,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material,
                        )));
                    }
                    _ => {
                        // Glass
                        let sphere_material = Dielectric::new(1.5);
                        scene.add(Box::new(MovingSphere::new(
                            center0,
                            center1,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material,
                        )));
                    }
                };
            }
        }
    }

    scene
}
