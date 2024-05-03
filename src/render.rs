use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    colour::RGB,
    hittable::{Hittable, HittableList},
    utilities::random,
    Camera, Ray, Resolution,
};

pub fn render_scene<F>(
    camera: &Camera,
    scene: &HittableList,
    resolution: &Resolution,
    report_progress: F,
) -> Vec<RGB>
where
    F: Fn(usize) + Sync,
{
    let mut pixels: Vec<(usize, usize)> =
        Vec::with_capacity(resolution.image_height * resolution.image_width);
    // Bottom -> top
    for j in (0..resolution.image_height).rev() {
        // Left -> right
        for i in 0..resolution.image_width {
            pixels.push((i, j))
        }
    }

    let image: Vec<RGB> = pixels
        .par_iter() // Parallel iteration
        .map(|pixel| {
            let mut colour = RGB(0.0, 0.0, 0.0);
            for _ in 0..resolution.num_samples {
                let u = ((pixel.0 as f64) + random()) / ((resolution.image_width - 1) as f64);
                let v = ((pixel.1 as f64) + random()) / ((resolution.image_height - 1) as f64);

                let ray = camera.get_ray(u, v);

                colour = colour + ray_colour(&ray, &scene, resolution.max_depth)
            }

            if pixel.0 == (resolution.image_width - 1) {
                // If we have just finished computing a row call report progress
                // with the row number of the row we have just finished
                report_progress(pixel.1)
            }
            colour
        })
        .collect();
    image
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
