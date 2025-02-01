use crate::{
    utilities::{degrees_to_radians, random_rng, random_vec_in_unit_disc},
    Point3d, Ray, Vec3d,
};

pub struct Camera {
    origin: Point3d,
    horizontal: Vec3d,
    vertical: Vec3d,
    lower_left_corner: Point3d,
    u: Vec3d,
    v: Vec3d,
    #[allow(dead_code)]
    w: Vec3d,
    lens_radius: f64,
    time0: f64, // shutter open time
    time1: f64, // shutter close time
}

impl Camera {
    /// - vertical_fov is the vertical field of view in degrees
    /// - view_up is the "up" direction for the camera, used to control the
    ///   roll/sideways tilt of the camera
    /// - time0 is the shutter open time
    /// - time1 is the shutter close time
    pub fn new(
        look_from: Point3d,
        look_at: Point3d,
        view_up: Vec3d,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (focus_dist * w);
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_vec_in_unit_disc();
        let offset = rd.x * self.u + rd.y * self.v;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            time: random_rng(self.time0, self.time1),
        }
    }
}
