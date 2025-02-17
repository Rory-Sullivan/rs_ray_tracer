use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    bvh::bvh::Bvh, camera::Camera, colour::RGB, hittable::hittable::Hittable, ray::Ray,
    resolution::Resolution, utilities::random,
};

pub fn render_scene<F>(
    camera: &Camera,
    bvh: &Bvh,
    resolution: &Resolution,
    report_progress: F,
    use_sky_background: bool,
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

                colour = colour + ray_colour(&ray, &bvh, resolution.max_depth, use_sky_background)
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

fn ray_colour(ray: &Ray, bvh: &Bvh, max_depth: usize, use_sky_background: bool) -> RGB {
    if max_depth <= 0 {
        return RGB(0.0, 0.0, 0.0);
    }

    let hit = bvh.hit(&ray, 0.001, f64::MAX);
    match hit {
        Some(hr) => match hr.material.scatter(ray, &hr) {
            Some((ray_out, hit_colour)) => {
                hr.material.emitted(hr.u, hr.v, hr.point)
                    + hit_colour * ray_colour(&ray_out, bvh, max_depth - 1, use_sky_background)
            }
            None => hr.material.emitted(hr.u, hr.v, hr.point),
        },
        None => {
            match use_sky_background {
                true => {
                    // Return sky colour based on direction of ray
                    let unit_direction = ray.direction.unit_vector();
                    let t = 0.5 * (unit_direction.y + 1.0);
                    (1.0 - t) * RGB(1.0, 1.0, 1.0) + t * RGB(0.5, 0.7, 1.0)
                }
                false => RGB(0.0, 0.0, 0.0), // Return background colour
            }
        }
    }
}
