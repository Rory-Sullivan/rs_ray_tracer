use std::{fs::create_dir_all, time::Instant};

use indicatif::ProgressBar;
use rs_ray_tracer::{
    bvh::Bvh,
    camera::Camera,
    colour::RGB,
    hittable::{HittableList, RotateY, Translate},
    material::{Dielectric, Diffuse, DiffuseLight, Lambertian, Metal},
    objects::{
        box_obj::BoxObj,
        moving_sphere::MovingSphere,
        rectangle::{RectangleXY, RectangleXZ, RectangleYZ},
        sphere::Sphere,
    },
    render::render_scene,
    resolution::Resolution,
    textures::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        perlin::Perlin, solid_colour::SolidColour, turbulence_texture::TurbulenceTexture,
    },
    utilities::{random, random_rgb, random_rng, random_vec_rng, save_as_png},
    vec3d::{Point3d, Vec3d},
    volumes::constant_medium::ConstantMedium,
};

fn main() {
    const OUTPUT_FOLDER: &str = "results";
    const OUTPUT_FILE_NAME: &str = "result";

    let start_instant = Instant::now();

    // Resolution
    let resolution = get_low_resolution();
    // let resolution = get_medium_resolution();
    // let resolution = get_high_resolution();

    // Cameras
    let t0 = 0.0; // Start time
    let t1 = 1.0; // Start time
    let cameras = get_final_scene_book2_camera(&resolution, t0, t1);

    // Scene
    let (scene, use_sky_background) = generate_final_scene_book2();
    let start_bvh_build_instant = Instant::now();
    let bvh = Bvh::build(scene.items, t0, t1);
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
        let image = render_scene(
            &camera,
            &bvh,
            &resolution,
            increment_progress_bar,
            use_sky_background,
        );

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

// Scenes
#[allow(dead_code)]
fn generate_basic_scene<'a>() -> (HittableList<'a>, bool) {
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

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_random_complex_scene<'a>() -> (HittableList<'a>, bool) {
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

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_random_complex_scene_moving_spheres<'a>() -> (HittableList<'a>, bool) {
    let mut scene = HittableList::new();

    // Use a checkered texture for the ground
    let checker_texture = CheckerTexture::new(
        Box::new(SolidColour::new(RGB(0.2, 0.3, 0.1))),
        Box::new(SolidColour::new(RGB(0.9, 0.9, 0.9))),
    );
    let material_ground = Lambertian::new(Box::new(checker_texture));
    let ground = Sphere::new(
        Point3d::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    );
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

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_two_checkered_spheres<'a>() -> (HittableList<'a>, bool) {
    let checker_texture = CheckerTexture::new(
        Box::new(SolidColour::new(RGB(0.2, 0.3, 0.1))),
        Box::new(SolidColour::new(RGB(0.9, 0.9, 0.9))),
    );
    let material_checker = Lambertian::new(Box::new(checker_texture));

    let sphere0 = Sphere::new(Vec3d::new(0.0, -10.0, 0.0), 10.0, material_checker.clone());
    let sphere1 = Sphere::new(Vec3d::new(0.0, 10.0, 0.0), 10.0, material_checker);

    let mut scene = HittableList::new();
    scene.add(Box::new(sphere0));
    scene.add(Box::new(sphere1));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_two_perlin_noise_spheres<'a>() -> (HittableList<'a>, bool) {
    let noise_texture = NoiseTexture::new(Perlin::build_random(), 4.0);
    let noise_material = Lambertian::new(Box::new(noise_texture));

    let sphere0 = Sphere::new(
        Vec3d::new(0.0, -1000.0, 0.0),
        1000.0,
        noise_material.clone(),
    );
    let sphere1 = Sphere::new(Vec3d::new(0.0, 2.0, 0.0), 2.0, noise_material);

    let mut scene = HittableList::new();
    scene.add(Box::new(sphere0));
    scene.add(Box::new(sphere1));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_two_perlin_noise_turbulence_spheres<'a>() -> (HittableList<'a>, bool) {
    let turbulence_texture = TurbulenceTexture::new(Perlin::build_random(), 4.0);
    let turbulence_material = Lambertian::new(Box::new(turbulence_texture));

    let sphere0 = Sphere::new(
        Vec3d::new(0.0, -1000.0, 0.0),
        1000.0,
        turbulence_material.clone(),
    );
    let sphere1 = Sphere::new(Vec3d::new(0.0, 2.0, 0.0), 2.0, turbulence_material);

    let mut scene = HittableList::new();
    scene.add(Box::new(sphere0));
    scene.add(Box::new(sphere1));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_earth_scene<'a>() -> (HittableList<'a>, bool) {
    let earth_texture = ImageTexture::build("images\\earthmap.jpg");
    let earth_material = Lambertian::new(Box::new(earth_texture));

    let earth = Sphere::new(Vec3d::new(0.0, 0.0, 0.0), 2.0, earth_material);

    let mut scene = HittableList::new();
    scene.add(Box::new(earth));

    let use_sky_background = true;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_simple_light<'a>() -> (HittableList<'a>, bool) {
    let turbulence_texture = TurbulenceTexture::new(Perlin::build_random(), 4.0);
    let turbulence_material = Lambertian::new(Box::new(turbulence_texture));

    let sphere0 = Sphere::new(
        Vec3d::new(-8.0, -1003.0, 0.0),
        1000.0,
        turbulence_material.clone(),
    );
    let sphere1 = Sphere::new(Vec3d::new(-8.0, -1.0, 0.0), 2.0, turbulence_material);

    // Note the light is brighter than (1, 1, 1) this allows it to light other
    // things.
    let diff_light = DiffuseLight::build_from_colour(RGB(4.0, 4.0, 4.0));
    let light_rect = RectangleXY::new(-5.0, -3.0, -2.0, 1.0, -2.0, diff_light.clone());
    let light_sphere = Sphere::new(Vec3d::new(-8.0, 3.0, 0.0), 1.0, diff_light);

    let mut scene = HittableList::new();
    scene.add(Box::new(sphere0));
    scene.add(Box::new(sphere1));
    scene.add(Box::new(light_rect));
    scene.add(Box::new(light_sphere));

    let use_sky_background = false;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_cornell_box<'a>() -> (HittableList<'a>, bool) {
    let time0 = 0.0;
    let time1 = 0.0;

    let red = Lambertian::build_from_colour(RGB(0.65, 0.05, 0.05));
    let green = Lambertian::build_from_colour(RGB(0.12, 0.45, 0.15));
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    let diffuse_light = DiffuseLight::build_from_colour(RGB(15.0, 15.0, 15.0));

    let red_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red);
    let green_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green);
    let light = RectangleXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, diffuse_light);
    let white_wall0 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone());
    let white_wall1 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());
    let white_wall2 = RectangleXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());

    let box0 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box0 = RotateY::new(15.0, Box::new(box0), time0, time1);
    let box0 = Translate::new(Vec3d::new(265.0, 0.0, 295.0), Box::new(box0));
    let box1 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 165.0, 165.0),
        white,
    );
    let box1 = RotateY::new(-18.0, Box::new(box1), time0, time1);
    let box1 = Translate::new(Vec3d::new(130.0, 0.0, 65.0), Box::new(box1));

    let mut scene = HittableList::new();
    scene.add(Box::new(red_wall));
    scene.add(Box::new(green_wall));
    scene.add(Box::new(light));
    scene.add(Box::new(white_wall0));
    scene.add(Box::new(white_wall1));
    scene.add(Box::new(white_wall2));
    scene.add(Box::new(box0));
    scene.add(Box::new(box1));

    let use_sky_background = false;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_cornell_box_with_smoke_boxes<'a>() -> (HittableList<'a>, bool) {
    let time0 = 0.0;
    let time1 = 0.0;

    let red = Lambertian::build_from_colour(RGB(0.65, 0.05, 0.05));
    let green = Lambertian::build_from_colour(RGB(0.12, 0.45, 0.15));
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    let diffuse_light = DiffuseLight::build_from_colour(RGB(7.0, 7.0, 7.0));

    let red_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red);
    let green_wall = RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, green);
    let white_wall0 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone());
    let white_wall1 = RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());
    let white_wall2 = RectangleXY::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());
    let light = RectangleXZ::new(113.0, 443.0, 127.0, 432.0, 554.0, diffuse_light); // larger dimmer light than standard Cornell

    let box0 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box0 = RotateY::new(15.0, Box::new(box0), time0, time1);
    let box0 = Translate::new(Vec3d::new(265.0, 0.0, 295.0), Box::new(box0));
    let box0 = ConstantMedium::build_from_colour(box0, RGB(0.0, 0.0, 0.0), 0.01); // light smoke box

    let box1 = BoxObj::new(
        Point3d::new(0.0, 0.0, 0.0),
        Point3d::new(165.0, 165.0, 165.0),
        white,
    );
    let box1 = RotateY::new(-18.0, Box::new(box1), time0, time1);
    let box1 = Translate::new(Vec3d::new(130.0, 0.0, 65.0), Box::new(box1));
    let box1 = ConstantMedium::build_from_colour(box1, RGB(1.0, 1.0, 1.0), 0.01); // dark smoke box

    let mut scene = HittableList::new();
    scene.add(Box::new(red_wall));
    scene.add(Box::new(green_wall));
    scene.add(Box::new(light));
    scene.add(Box::new(white_wall0));
    scene.add(Box::new(white_wall1));
    scene.add(Box::new(white_wall2));
    scene.add(Box::new(box0));
    scene.add(Box::new(box1));

    let use_sky_background = false;

    (scene, use_sky_background)
}

#[allow(dead_code)]
fn generate_final_scene_book2<'a>() -> (HittableList<'a>, bool) {
    let time0 = 0.0;
    let time1 = 1.0;
    let use_sky_background = false;
    let mut scene = HittableList::new();

    // Make the ground a 20x20 grid of random height boxes
    let mut ground_boxes = HittableList::new();
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

            ground_boxes.add(Box::new(BoxObj::new(
                Point3d::new(x0, y0, z0),
                Point3d::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    scene.add(Box::new(Bvh::build(ground_boxes.items, time0, time1)));

    // Make a light
    let diffuse_light = DiffuseLight::build_from_colour(RGB(7.0, 7.0, 7.0));
    let light = RectangleXZ::new(123.0, 423.0, 147.0, 412.0, 554.0, diffuse_light);
    scene.add(Box::new(light));

    // Make a moving sphere
    let center0 = Point3d::new(400.0, 400.0, 200.0);
    let center1 = center0 + Vec3d::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::build_from_colour(RGB(0.7, 0.3, 0.1));
    let moving_sphere =
        MovingSphere::new(center0, center1, time0, time1, 50.0, moving_sphere_material);
    scene.add(Box::new(moving_sphere));

    // Add a dielectric (glass) sphere
    let sphere0 = Sphere::new(Point3d::new(260.0, 150.0, 45.0), 50.0, Dielectric::new(1.5));
    scene.add(Box::new(sphere0));

    // Add a metal sphere
    let sphere1 = Sphere::new(
        Point3d::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(RGB(0.8, 0.8, 0.9), 1.0),
    );
    scene.add(Box::new(sphere1));

    // Add a blue subsurface reflection sphere by putting a volume inside a
    // dielectric sphere.
    let boundary0 = Sphere::new(
        Point3d::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5),
    );
    let smoke_sphere0 = ConstantMedium::build_from_colour(boundary0, RGB(0.2, 0.4, 0.9), 0.2);
    scene.add(Box::new(boundary0));
    scene.add(Box::new(smoke_sphere0));

    // Fill the whole scene with a faint mist
    let boundary1 = Sphere::new(Point3d::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    let smoke_sphere1 = ConstantMedium::build_from_colour(boundary0, RGB(1.0, 1.0, 1.0), 0.0001);
    scene.add(Box::new(boundary1));
    scene.add(Box::new(smoke_sphere1));

    // Add an Earth sphere
    let earth_material = Lambertian::new(Box::new(ImageTexture::build("images\\earthmap.jpg")));
    let earth_sphere = Sphere::new(Vec3d::new(400.0, 200.0, 400.0), 100.0, earth_material);
    scene.add(Box::new(earth_sphere));

    // Add a perlin noise sphere
    let perlin_texture = TurbulenceTexture::new(Perlin::build_random(), 0.001);
    let perlin_material = Lambertian::new(Box::new(perlin_texture));
    let perlin_sphere = Sphere::new(Point3d::new(220.0, 280.0, 300.0), 80.0, perlin_material);
    scene.add(Box::new(perlin_sphere));

    // Add a random assortment of white spheres in a translated rotated box
    let mut spheres = HittableList::new();
    let white = Lambertian::build_from_colour(RGB(0.73, 0.73, 0.73));
    for _ in 0..1000 {
        let sphere = Sphere::new(random_vec_rng(0.0, 165.0), 10.0, white.clone());
        spheres.add(Box::new(sphere));
    }
    let translated_rotated_bvh_of_spheres = Translate::new(
        Point3d::new(-100.0, 270.0, 395.0),
        Box::new(RotateY::new(
            15.0,
            Box::new(Bvh::build(spheres.items, time0, time1)),
            time0,
            time1,
        )),
    );
    scene.add(Box::new(translated_rotated_bvh_of_spheres));

    (scene, use_sky_background)
}
