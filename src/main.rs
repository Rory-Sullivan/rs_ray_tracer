use std::{fs::create_dir_all, time::Instant};

use indicatif::ProgressBar;
use rs_ray_tracer::{
    bvh::bvh::Bvh,
    camera::Camera,
    colour::RGB,
    hittable::hittable::Hittable,
    instances::*,
    materials::*,
    objects::*,
    render::render_scene,
    resolution::Resolution,
    textures::*,
    utilities::{random, random_rgb, random_rng, random_vec_rng, save_as_png},
    vec3d::{Point3d, Vec3d},
    volumes::constant_medium::ConstantMedium,
};

fn main() {
    const OUTPUT_FOLDER: &str = "results";
    const OUTPUT_FILE_NAME: &str = "result";

    let start_instant = Instant::now();
    let start_scene_build_instant = Instant::now();

    // Resolution
    // let resolution = get_cornell_square_resolution();
    let resolution = get_low_resolution();
    // let resolution = get_medium_resolution();
    // let resolution = get_high_resolution();

    // Cameras
    let time0 = 0.0; // Start time
    let time1 = 1.0; // End time
    let cameras = get_final_scene_cameras(&resolution, time0, time1);

    // Scene
    let (scene, use_sky_background) = generate_final_scene();
    let (bvh, bvh_metrics) = Bvh::build(time0, time1, scene);
    print_time_taken("Done building scene", start_scene_build_instant);
    println!("Main BVH metrics: {bvh_metrics:?}");

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
        let image = render_scene(
            camera,
            &bvh,
            &resolution,
            increment_progress_bar,
            use_sky_background,
        );

        progress_bar.finish();
        println!();

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

// Resolutions
#[allow(dead_code)]
fn get_low_resolution() -> Resolution {
    Resolution::new(
        600, // Image width
        400, // Image height
        500, // Num samples
        50,  // Max depth
    )
}

#[allow(dead_code)]
fn get_cornell_square_resolution() -> Resolution {
    Resolution::new(
        400, // Image width
        400, // Image height
        500, // Num samples
        50,  // Max depth
    )
}

#[allow(dead_code)]
fn get_medium_resolution() -> Resolution {
    Resolution::new(
        1200, // Image width
        800,  // Image height
        500,  // Num samples
        50,   // Max depth
    )
}

#[allow(dead_code)]
fn get_high_resolution() -> Resolution {
    Resolution::new(
        1920,  // Image width
        1080,  // Image height
        10000, // Num samples
        50,    // Max depth
    )
}

// Camera setups
#[allow(dead_code)]
fn get_standard_camera(resolution: &Resolution, t0: f64, t1: f64) -> Vec<Camera> {
    vec![Camera::new(
        Point3d::new(13.0, 2.0, 3.0),  // Look from
        Point3d::new(0.0, 0.0, 0.0),   // Look at
        Vec3d::new(0.0, 1.0, 0.0),     // View up (the up direction of the camera)
        20.0,                          // Vertical field of view in degrees
        resolution.get_aspect_ratio(), // Aspect ratio
        0.1,                           // Aperture
        10.0,                          // Focus distance
        t0,                            // Start time
        t1,                            // End time
    )]
}

#[allow(dead_code)]
fn get_standard_multi_cameras(resolution: &Resolution, t0: f64, t1: f64) -> Vec<Camera> {
    // Camera
    const FOV: f64 = 20.0; // degrees
    const APERTURE: f64 = 0.1;

    vec![
        Camera::new(
            Point3d::new(13.0, 2.0, 3.0),  // Look from
            Point3d::new(0.0, 0.0, 0.0),   // Look at
            Vec3d::new(0.0, 1.0, 0.0),     // View up
            FOV,                           // Vertical field of view
            resolution.get_aspect_ratio(), // Aspect ratio
            APERTURE,                      // Aperture
            10.0,                          // Focus distance
            t0,                            // Start time
            t1,                            // End time
        ),
        Camera::new(
            Point3d::new(5.0, 5.0, 13.0),  // Look from
            Point3d::new(0.0, 0.0, 0.0),   // Look at
            Vec3d::new(0.0, 1.0, 0.0),     // View up
            FOV,                           // Vertical field of view
            resolution.get_aspect_ratio(), // Aspect ratio
            APERTURE,                      // Aperture
            13.3,                          // Focus distance
            t0,                            // Start time
            t1,                            // End time
        ),
        Camera::new(
            Point3d::new(-6.0, 1.0, -10.0), // Look from
            Point3d::new(4.0, 0.0, 0.0),    // Look at
            Vec3d::new(0.0, 1.0, 0.0),      // View up
            FOV,                            // Vertical field of view
            resolution.get_aspect_ratio(),  // Aspect ratio
            APERTURE,                       // Aperture
            16.0,                           // Focus distance
            t0,                             // Start time
            t1,                             // End time
        ),
    ]
}

#[allow(dead_code)]
fn get_cornell_box_camera(resolution: &Resolution, t0: f64, t1: f64) -> Vec<Camera> {
    vec![Camera::new(
        Point3d::new(278.0, 278.0, -800.0), // Look from
        Point3d::new(278.0, 278.0, 0.0),    // Look at
        Vec3d::new(0.0, 1.0, 0.0),          // View up (the up direction of the camera)
        40.0,                               // Vertical field of view in degrees
        resolution.get_aspect_ratio(),      // Aspect ratio
        0.0,                                // Aperture
        10.0,                               // Focus distance
        t0,                                 // Start time
        t1,                                 // End time
    )]
}

#[allow(dead_code)]
fn get_final_scene_book2_camera(resolution: &Resolution, t0: f64, t1: f64) -> Vec<Camera> {
    vec![Camera::new(
        Point3d::new(478.0, 278.0, -600.0), // Look from
        Point3d::new(278.0, 278.0, 0.0),    // Look at
        Vec3d::new(0.0, 1.0, 0.0),          // View up (the up direction of the camera)
        40.0,                               // Vertical field of view in degrees
        resolution.get_aspect_ratio(),      // Aspect ratio
        0.0,                                // Aperture
        10.0,                               // Focus distance
        t0,                                 // Start time
        t1,                                 // End time
    )]
}

#[allow(dead_code)]
fn get_final_scene_cameras(resolution: &Resolution, t0: f64, t1: f64) -> Vec<Camera> {
    vec![
        Camera::new(
            Point3d::new(478.0, 278.0, -600.0), // Look from
            Point3d::new(200.0, 278.0, 280.0),  // Look at
            Vec3d::new(0.0, 1.0, 0.0),          // View up (the up direction of the camera)
            40.0,                               // Vertical field of view in degrees
            resolution.get_aspect_ratio(),      // Aspect ratio
            0.0,                                // Aperture
            922.0,                              // Focus distance
            t0,                                 // Start time
            t1,                                 // End time
        ),
        // Camera::new(
        //     Point3d::new(0.0, 278.0, -600.0),  // Look from
        //     Point3d::new(200.0, 278.0, 280.0), // Look at
        //     Vec3d::new(0.0, 1.0, 0.0),         // View up (the up direction of the camera)
        //     40.0,                              // Vertical field of view in degrees
        //     resolution.get_aspect_ratio(),     // Aspect ratio
        //     0.0,                               // Aperture
        //     10.0,                              // Focus distance
        //     t0,                                // Start time
        //     t1,                                // End time
        // ),
        // Camera::new(
        //     Point3d::new(232.0, 478.0, -600.0), // Look from
        //     Point3d::new(200.0, 278.0, 280.0),  // Look at
        //     Vec3d::new(0.0, 1.0, 0.0),          // View up (the up direction of the camera)
        //     40.0,                               // Vertical field of view in degrees
        //     resolution.get_aspect_ratio(),      // Aspect ratio
        //     0.0,                                // Aperture
        //     10.0,                               // Focus distance
        //     t0,                                 // Start time
        //     t1,                                 // End time
        // ),
    ]
}

// Scenes
#[allow(dead_code)]
fn generate_basic_scene() -> (Vec<Box<dyn Hittable>>, bool) {
    // Basic scene
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Diffuse::new(RGB(0.8, 0.8, 0.0));
    let material_centre = Diffuse::new(RGB(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(RGB(0.8, 0.6, 0.2), 0.0);

    let ground = Sphere::new(Vec3d::new(0.0, -100.5, 1.0), 100.0, material_ground);
    let centre_sphere = Sphere::new(Vec3d::new(0.0, 0.0, -1.0), 0.5, material_centre);
    let left_sphere = Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let left_inner_sphere = Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), -0.45, material_left);
    let right_sphere = Sphere::new(Vec3d::new(1.0, 0.0, -1.0), 0.5, material_right);

    scene.push(Box::new(ground));
    scene.push(Box::new(centre_sphere));
    scene.push(Box::new(left_sphere));
    scene.push(Box::new(left_inner_sphere));
    scene.push(Box::new(right_sphere));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_random_complex_scene() -> (Vec<Box<dyn Hittable>>, bool) {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();
    let material_ground = Diffuse::new(RGB(0.5, 0.5, 0.5));
    let ground = Sphere::new(Point3d::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    scene.push(Box::new(ground));

    // Add three large spheres
    let material1 = Dielectric::new(1.5);
    scene.push(Box::new(Sphere::new(
        Point3d::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Diffuse::new(RGB(0.4, 0.2, 0.1));
    scene.push(Box::new(Sphere::new(
        Point3d::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::new(RGB(0.7, 0.6, 0.5), 0.0);
    scene.push(Box::new(Sphere::new(
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
                        scene.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    x if x < 0.95 => {
                        // Metal
                        let albedo = random_rgb();
                        let fuzz = random_rng(0.0, 0.5);
                        let sphere_material = Metal::new(albedo, fuzz);
                        scene.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // Glass
                        let sphere_material = Dielectric::new(1.5);
                        scene.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                };
            }
        }
    }

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_random_complex_scene_moving_spheres() -> (Vec<Box<dyn Hittable>>, bool) {
    let time0 = 0.0;
    let time1 = 1.0;

    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    // Use a checkered texture for the ground
    let checker_texture = CheckerTexture::new(
        SolidColour::new(RGB(0.2, 0.3, 0.1)),
        SolidColour::new(RGB(0.9, 0.9, 0.9)),
    );
    let material_ground = Lambertian::new(checker_texture);
    let ground = Sphere::new(Point3d::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
    scene.push(Box::new(ground));

    // Add three large spheres
    let material1 = Dielectric::new(1.5);
    scene.push(Box::new(Sphere::new(
        Point3d::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Diffuse::new(RGB(0.4, 0.2, 0.1));
    scene.push(Box::new(Sphere::new(
        Point3d::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::new(RGB(0.7, 0.6, 0.5), 0.0);
    scene.push(Box::new(Sphere::new(
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
                        scene.push(Box::new(MovingSphere::new(
                            center0,
                            center1,
                            time0,
                            time1,
                            0.2,
                            sphere_material,
                        )));
                    }
                    x if x < 0.95 => {
                        // Metal
                        let albedo = random_rgb();
                        let fuzz = random_rng(0.0, 0.5);
                        let sphere_material = Metal::new(albedo, fuzz);
                        scene.push(Box::new(MovingSphere::new(
                            center0,
                            center1,
                            time0,
                            time1,
                            0.2,
                            sphere_material,
                        )));
                    }
                    _ => {
                        // Glass
                        let sphere_material = Dielectric::new(1.5);
                        scene.push(Box::new(MovingSphere::new(
                            center0,
                            center1,
                            time0,
                            time1,
                            0.2,
                            sphere_material,
                        )));
                    }
                };
            }
        }
    }

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_two_checkered_spheres() -> (Vec<Box<dyn Hittable>>, bool) {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let checker_texture = CheckerTexture::new(
        SolidColour::new(RGB(0.2, 0.3, 0.1)),
        SolidColour::new(RGB(0.9, 0.9, 0.9)),
    );
    let material_checker = Lambertian::new(checker_texture);

    let sphere0 = Sphere::new(Vec3d::new(0.0, -10.0, 0.0), 10.0, material_checker);
    let sphere1 = Sphere::new(Vec3d::new(0.0, 10.0, 0.0), 10.0, material_checker);

    scene.push(Box::new(sphere0));
    scene.push(Box::new(sphere1));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_two_perlin_noise_spheres() -> (Vec<Box<dyn Hittable>>, bool) {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let noise_texture = NoiseTexture::new(Perlin::build_random(), 4.0);
    let noise_material = Lambertian::new(noise_texture);

    let sphere0 = Sphere::new(
        Vec3d::new(0.0, -1000.0, 0.0),
        1000.0,
        noise_material.clone(),
    );
    let sphere1 = Sphere::new(Vec3d::new(0.0, 2.0, 0.0), 2.0, noise_material);

    scene.push(Box::new(sphere0));
    scene.push(Box::new(sphere1));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_two_perlin_noise_turbulence_spheres() -> (Vec<Box<dyn Hittable>>, bool) {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let turbulence_texture = TurbulenceTexture::new(Perlin::build_random(), 4.0);
    let turbulence_material = Lambertian::new(turbulence_texture);

    let sphere0 = Sphere::new(
        Vec3d::new(0.0, -1000.0, 0.0),
        1000.0,
        turbulence_material.clone(),
    );
    let sphere1 = Sphere::new(Vec3d::new(0.0, 2.0, 0.0), 2.0, turbulence_material);

    scene.push(Box::new(sphere0));
    scene.push(Box::new(sphere1));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_earth_scene() -> (Vec<Box<dyn Hittable>>, bool) {
    let earth_texture = ImageTexture::build("images\\earthmap.jpg");
    let earth_material = Lambertian::new(earth_texture);

    let earth = Sphere::new(Vec3d::new(0.0, 0.0, 0.0), 2.0, earth_material);

    let scene: Vec<Box<dyn Hittable>> = vec![Box::new(earth)];

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_simple_light() -> (Vec<Box<dyn Hittable>>, bool) {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let turbulence_texture = TurbulenceTexture::new(Perlin::build_random(), 4.0);
    let turbulence_material = Lambertian::new(turbulence_texture);

    let sphere0 = Sphere::new(
        Vec3d::new(-8.0, -1003.0, 0.0),
        1000.0,
        turbulence_material.clone(),
    );
    let sphere1 = Sphere::new(Vec3d::new(-8.0, -1.0, 0.0), 2.0, turbulence_material);

    // Note the light is brighter than (1, 1, 1) this allows it to light other
    // things.
    let diff_light = DiffuseLight::build_from_colour(RGB(4.0, 4.0, 4.0));
    let light_rect = RectangleXY::new(-5.0, -3.0, -2.0, 1.0, -2.0, diff_light);
    let light_sphere = Sphere::new(Vec3d::new(-8.0, 3.0, 0.0), 1.0, diff_light);

    scene.push(Box::new(sphere0));
    scene.push(Box::new(sphere1));
    scene.push(Box::new(light_rect));
    scene.push(Box::new(light_sphere));

    let use_sky_background = false;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_cornell_box() -> (Vec<Box<dyn Hittable>>, bool) {
    let time0 = 0.0;
    let time1 = 0.0;
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let red = Lambertian::build_from_colour(RGB(0.65, 0.05, 0.05));
    let green = Lambertian::build_from_colour(RGB(0.12, 0.45, 0.15));
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    let diffuse_light = DiffuseLight::build_from_colour(RGB(15.0, 15.0, 15.0));

    let red_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red);
    let green_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green);
    let light = RectangleXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light);
    let white_wall0 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white);
    let white_wall1 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white);
    let white_wall2 = RectangleXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white);

    let box0 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 330.0, 165.0),
        white,
    );
    let box0 = RotateY::new(15.0, box0, time0, time1);
    let box0 = Translate::new(Vec3d::new(265.0, 0.0, 295.0), box0);
    let box1 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 165.0, 165.0),
        white,
    );
    let box1 = RotateY::new(-18.0, box1, time0, time1);
    let box1 = Translate::new(Vec3d::new(130.0, 0.0, 65.0), box1);

    scene.push(Box::new(red_wall));
    scene.push(Box::new(green_wall));
    scene.push(Box::new(light));
    scene.push(Box::new(white_wall0));
    scene.push(Box::new(white_wall1));
    scene.push(Box::new(white_wall2));
    scene.push(Box::new(box0));
    scene.push(Box::new(box1));

    let use_sky_background = false;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_cornell_box_with_smoke_boxes() -> (Vec<Box<dyn Hittable>>, bool) {
    let time0 = 0.0;
    let time1 = 0.0;
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let red = Lambertian::build_from_colour(RGB(0.65, 0.05, 0.05));
    let green = Lambertian::build_from_colour(RGB(0.12, 0.45, 0.15));
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    let diffuse_light = DiffuseLight::build_from_colour(RGB(7.0, 7.0, 7.0));

    let red_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red);
    let green_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green);
    let white_wall0 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white);
    let white_wall1 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white);
    let white_wall2 = RectangleXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white);
    let light = RectangleXZ::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light); // larger dimmer light than standard Cornell

    let box0 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 330.0, 165.0),
        white,
    );
    let box0 = RotateY::new(15.0, box0, time0, time1);
    let box0 = Translate::new(Vec3d::new(265.0, 0.0, 295.0), box0);
    let box0 = ConstantMedium::build_from_colour(box0, RGB(0.0, 0.0, 0.0), 0.01); // light smoke box

    let box1 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 165.0, 165.0),
        white,
    );
    let box1 = RotateY::new(-18.0, box1, time0, time1);
    let box1 = Translate::new(Vec3d::new(130.0, 0.0, 65.0), box1);
    let box1 = ConstantMedium::build_from_colour(box1, RGB(1.0, 1.0, 1.0), 0.01); // dark smoke box

    scene.push(Box::new(red_wall));
    scene.push(Box::new(green_wall));
    scene.push(Box::new(light));
    scene.push(Box::new(white_wall0));
    scene.push(Box::new(white_wall1));
    scene.push(Box::new(white_wall2));
    scene.push(Box::new(box0));
    scene.push(Box::new(box1));

    let use_sky_background = false;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_final_scene_book2() -> (Vec<Box<dyn Hittable>>, bool) {
    let time0 = 0.0;
    let time1 = 1.0;
    let use_sky_background = false;
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    // Make the ground a 20x20 grid of random height boxes
    let mut ground_boxes: Vec<Box<dyn Hittable>> = Vec::new();
    let ground = Lambertian::build_from_colour(RGB(0.48, 0.83, 0.53));
    for i in 0..20 {
        for j in 0..20 {
            let width = 100.0;
            let x0 = -1000.0 + (i as f64) * width;
            let y0 = 0.0;
            let z0 = -1000.0 + (j as f64) * width;
            let x1 = x0 + width;
            let y1 = random_rng(1.0, 101.0);
            let z1 = z0 + width;

            ground_boxes.push(Box::new(BoxObj::new(
                Point3d::new(x0, y0, z0),
                Point3d::new(x1, y1, z1),
                ground,
            )));
        }
    }
    scene.push(Box::new(Bvh::build(time0, time1, ground_boxes).0));

    // Make a light
    let diffuse_light = DiffuseLight::build_from_colour(RGB(7.0, 7.0, 7.0));
    let light = RectangleXZ::new(123.0, 423.0, 147.0, 412.0, 554.0, diffuse_light);
    scene.push(Box::new(light));

    // Make a moving sphere
    let center0 = Point3d::new(400.0, 400.0, 200.0);
    let center1 = center0 + Vec3d::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::build_from_colour(RGB(0.7, 0.3, 0.1));
    let moving_sphere =
        MovingSphere::new(center0, center1, time0, time1, 50.0, moving_sphere_material);
    scene.push(Box::new(moving_sphere));

    // Add a dielectric (glass) sphere
    let sphere0 = Sphere::new(Point3d::new(260.0, 150.0, 45.0), 50.0, Dielectric::new(1.5));
    scene.push(Box::new(sphere0));

    // Add a metal sphere
    let sphere1 = Sphere::new(
        Point3d::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(RGB(0.8, 0.8, 0.9), 1.0),
    );
    scene.push(Box::new(sphere1));

    // Add a blue subsurface reflection sphere by putting a volume inside a
    // dielectric sphere.
    let boundary0 = Sphere::new(
        Point3d::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5),
    );
    let smoke_sphere0 = ConstantMedium::build_from_colour(boundary0, RGB(0.2, 0.4, 0.9), 0.2);
    scene.push(Box::new(boundary0));
    scene.push(Box::new(smoke_sphere0));

    // Fill the whole scene with a faint mist
    let boundary1 = Sphere::new(Point3d::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    let smoke_sphere1 = ConstantMedium::build_from_colour(boundary0, RGB(1.0, 1.0, 1.0), 0.0001);
    scene.push(Box::new(boundary1));
    scene.push(Box::new(smoke_sphere1));

    // Add an Earth sphere
    let earth_material = Lambertian::new(ImageTexture::build("images\\earthmap.jpg"));
    let earth_sphere = Sphere::new(Vec3d::new(400.0, 200.0, 400.0), 100.0, earth_material);
    scene.push(Box::new(earth_sphere));

    // Add a perlin noise sphere
    let perlin_texture = TurbulenceTexture::new(Perlin::build_random(), 0.001);
    let perlin_material = Lambertian::new(perlin_texture);
    let perlin_sphere = Sphere::new(Point3d::new(220.0, 280.0, 300.0), 80.0, perlin_material);
    scene.push(Box::new(perlin_sphere));

    // Add a random assortment of white spheres in a translated rotated box
    let mut spheres: Vec<Box<dyn Hittable>> = Vec::new();
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    for _ in 0..1000 {
        let sphere = Sphere::new(random_vec_rng(0.0, 165.0), 10.0, white);
        spheres.push(Box::new(sphere));
    }
    let translated_rotated_bvh_of_spheres = Translate::new(
        Point3d::new(-100.0, 270.0, 395.0),
        RotateY::new(15.0, Bvh::build(time0, time1, spheres).0, time0, time1),
    );
    scene.push(Box::new(translated_rotated_bvh_of_spheres));

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_cornell_box_with_pyramids() -> (Vec<Box<dyn Hittable>>, bool) {
    let time0 = 0.0;
    let time1 = 0.0;
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let red = Lambertian::build_from_colour(RGB(0.65, 0.05, 0.05));
    let green = Lambertian::build_from_colour(RGB(0.12, 0.45, 0.15));
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    let diffuse_light = DiffuseLight::build_from_colour(RGB(12.0, 12.0, 12.0));

    let red_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red);
    let green_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green);
    let light = RectangleXZ::new(163.0, 393.0, 177.0, 382.0, 554.0, diffuse_light);
    let white_wall0 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white);
    let white_wall1 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white);
    let white_wall2 = RectangleXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white);

    let pyr0 = Pyramid::build(
        (
            Point3d::new(0.0, 0.0, 0.0),
            Point3d::new(200.0, 0.0, 0.0),
            Point3d::new(0.0, 0.0, 200.0),
        ),
        330.0,
        white,
    );
    let pyr0 = RotateY::new(15.0, pyr0, time0, time1);
    let pyr0 = Translate::new(Vec3d::new(265.0, 0.0, 295.0), pyr0);
    let pyr1 = Pyramid::build(
        (
            Point3d::new(0.0, 0.0, 0.0),
            Point3d::new(200.0, 0.0, 0.0),
            Point3d::new(0.0, 0.0, 200.0),
        ),
        165.0,
        white,
    );
    let pyr1 = RotateY::new(-18.0, pyr1, time0, time1);
    let pyr1 = Translate::new(Vec3d::new(130.0, 0.0, 65.0), pyr1);

    scene.push(Box::new(red_wall));
    scene.push(Box::new(green_wall));
    scene.push(Box::new(light));
    scene.push(Box::new(white_wall0));
    scene.push(Box::new(white_wall1));
    scene.push(Box::new(white_wall2));
    scene.push(Box::new(pyr0));
    scene.push(Box::new(pyr1));

    let use_sky_background = false;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_cornell_box_with_dragon() -> (Vec<Box<dyn Hittable>>, bool) {
    let time0 = 0.0;
    let time1 = 0.0;
    let use_sky_background = false;
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let dragon_material = Metal::new(RGB::from_hash("#ffd700"), 0.8); // #ffd700
    let (dragon, dragon_metrics) =
        Model::build("assets/stanford_dragon/dragon_vrip.ply", dragon_material);
    println!("Dragon metrics: {dragon_metrics:?}");
    let dragon = Scale::new(2600.0, 2600.0, 2600.0, dragon);
    let dragon = RotateY::new(-167.0, dragon, time0, time1);
    let dragon = Translate::new(Vec3d::new(265.0, -140.0, 295.0), dragon);
    scene.push(Box::new(dragon));

    // Add three white walls (top, back, bottom)
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    let white_wall0 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white);
    let white_wall1 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white);
    let white_wall2 = RectangleXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white);
    scene.push(Box::new(white_wall0));
    scene.push(Box::new(white_wall1));
    scene.push(Box::new(white_wall2));

    // Add right red wall
    let red = Lambertian::build_from_colour(RGB(0.65, 0.05, 0.05));
    let red_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red);
    scene.push(Box::new(red_wall));

    // Add left green wall
    let green = Lambertian::build_from_colour(RGB(0.12, 0.45, 0.15));
    let green_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green);
    scene.push(Box::new(green_wall));

    // Add light
    let diffuse_light = DiffuseLight::build_from_colour(RGB(12.0, 12.0, 12.0));
    let light = RectangleXZ::new(163.0, 393.0, 177.0, 382.0, 554.0, diffuse_light);
    scene.push(Box::new(light));

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_final_scene() -> (Vec<Box<dyn Hittable>>, bool) {
    let time0 = 0.0;
    let time1 = 1.0;
    let use_sky_background = false;
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    // Make the ground a 20x20 grid of random height boxes with a platform in the middle
    // box width: 100
    // x in [-1000, 1000]
    // z in [-1000, 1000]
    let mut ground_boxes: Vec<Box<dyn Hittable>> = Vec::new();
    let ground = Lambertian::build_from_colour(RGB(0.48, 0.83, 0.53));
    let width = 100.0;
    for i in 0..20 {
        for j in 0..20 {
            let x0 = -1000.0 + (i as f64) * width;
            let y0 = 0.0;
            let z0 = -1000.0 + (j as f64) * width;
            let x1 = x0 + width;
            let y1 = match (i, j) {
                (i, j) if (10..14).contains(&i) && (10..14).contains(&j) => 100.0,
                _ => random_rng(1.0, 96.0),
            };
            let z1 = z0 + width;

            ground_boxes.push(Box::new(BoxObj::new(
                Point3d::new(x0, y0, z0),
                Point3d::new(x1, y1, z1),
                ground,
            )));
        }
    }
    scene.push(Box::new(Bvh::build(time0, time1, ground_boxes).0));

    // Make a light
    // Center: (200.0, 554.0, 280.0)
    let diffuse_light = DiffuseLight::build_from_colour(RGB(7.0, 7.0, 7.0));
    let light = RectangleXZ::new(
        -123.0, // x0
        523.0,  // x1
        147.0,  // z0
        412.0,  // z1
        554.0,  // k
        diffuse_light,
    );
    scene.push(Box::new(light));

    // Make a gold dragon
    let dragon_material = Metal::new(RGB::from_hash("#ffd700"), 0.8); // #ffd700
    let (dragon, dragon_metrics) =
        Model::build("assets/stanford_dragon/dragon_vrip.ply", dragon_material);
    println!("Dragon metrics: {dragon_metrics:?}");
    let dragon = Scale::new(2000.0, 2000.0, 2000.0, dragon);
    let dragon = RotateY::new(-167.0, dragon, time0, time1);
    let dragon = Translate::new(Vec3d::new(200.0, -10.0, 200.0), dragon);
    scene.push(Box::new(dragon));

    // Add a moving sphere
    let center0 = Point3d::new(430.0, 400.0, 500.0);
    let center1 = center0 + Vec3d::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::build_from_colour(RGB(0.7, 0.3, 0.1));
    let moving_sphere =
        MovingSphere::new(center0, center1, time0, time1, 50.0, moving_sphere_material);
    scene.push(Box::new(moving_sphere));

    // Add a dielectric (glass) sphere
    let sphere0 = Sphere::new(Point3d::new(175.0, 150.0, 45.0), 50.0, Dielectric::new(1.5));
    scene.push(Box::new(sphere0));

    // Add a red pyramid
    let pyr0 = Pyramid::build(
        (
            Point3d::new(0.0, 0.0, 0.0),
            Point3d::new(0.0, 0.0, 75.0),
            Point3d::new(75.0, 0.0, 0.0),
        ),
        75.0,
        Lambertian::build_from_colour(RGB(0.65, 0.05, 0.05)),
    );
    let pyr0 = RotateY::new(38.0, pyr0, time0, time1);
    let pyr0 = Translate::new(Vec3d::new(8.0, 100.0, 142.0), pyr0); // Point3d::new(0.0, 150.0, 145.0),
    scene.push(Box::new(pyr0));

    // Add a metal sphere
    let sphere1 = Sphere::new(
        Point3d::new(-50.0, 150.0, 156.0),
        50.0,
        Metal::new(RGB(0.8, 0.8, 0.9), 0.1),
    );
    scene.push(Box::new(sphere1));

    // Add a blue subsurface reflection sphere by putting a volume inside a
    // dielectric sphere.
    let boundary0 = Sphere::new(
        Point3d::new(460.0, 160.0, 145.0),
        60.0,
        Dielectric::new(1.5),
    );
    let smoke_sphere0 = ConstantMedium::build_from_colour(boundary0, RGB(0.2, 0.4, 0.9), 0.1);
    scene.push(Box::new(boundary0));
    scene.push(Box::new(smoke_sphere0));

    // Add an Earth sphere
    let earth_material = Lambertian::new(ImageTexture::build("images\\earthmap.jpg"));
    let earth_sphere = Sphere::new(Vec3d::new(0.0, 0.0, 0.0), 100.0, earth_material);
    let earth_sphere = RotateY::new(78.0, earth_sphere, time0, time1);
    let earth_sphere = Translate::new(Vec3d::new(500.0, 200.0, 400.0), earth_sphere);
    scene.push(Box::new(earth_sphere));

    // Add a perlin noise sphere
    let perlin_texture = TurbulenceTexture::new(Perlin::build_random(), 2.0);
    let perlin_material = Lambertian::new(perlin_texture);
    let perlin_sphere = Sphere::new(Point3d::new(82.0, 370.0, 484.0), 80.0, perlin_material);
    scene.push(Box::new(perlin_sphere));

    // Add a random assortment of white spheres in a translated rotated box
    let mut spheres: Vec<Box<dyn Hittable>> = Vec::new();
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    for _ in 0..1000 {
        let sphere = Sphere::new(random_vec_rng(0.0, 165.0), 10.0, white);
        spheres.push(Box::new(sphere));
    }
    let translated_rotated_bvh_of_spheres = Translate::new(
        Point3d::new(-250.0, 270.0, 395.0),
        RotateY::new(15.0, Bvh::build(time0, time1, spheres).0, time0, time1),
    );
    scene.push(Box::new(translated_rotated_bvh_of_spheres));

    // Add coordinate spheres for debugging positions
    // let sphere_origin = Sphere::new(
    //     Point3d::new(0.0, 100.0, 0.0),
    //     20.0,
    //     Diffuse::new(RGB(0.73, 0.73, 0.73)), // white
    // );
    // scene.push(Box::new(sphere_origin));

    // let sphere_x = Sphere::new(
    //     Point3d::new(200.0, 100.0, 0.0),
    //     20.0,
    //     Diffuse::new(RGB(0.73, 0.05, 0.05)), // red
    // );
    // scene.push(Box::new(sphere_x));

    // let sphere_z = Sphere::new(
    //     Point3d::new(0.0, 100.0, 200.0),
    //     20.0,
    //     Diffuse::new(RGB(0.05, 0.73, 0.05)), // green
    // );
    // scene.push(Box::new(sphere_z));

    (scene, use_sky_background)
}
