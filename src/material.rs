use crate::{
    colour::RGB,
    hittable::HitRecord,
    utilities::{random, random_unit_vec, random_vec_in_unit_sphere, reflect_vec, refract_vec},
    Ray,
};

pub trait Material {
    // Returns scattered ray and an attenuation colour
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Diffuse {
    pub albedo: RGB,
}

impl Diffuse {
    pub fn new(albedo: RGB) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let mut scatter_direction = hit_record.normal + random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }
        let ray_out = Ray::new(hit_record.point, scatter_direction, ray_in.time);
        return Some((ray_out, self.albedo));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: RGB,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: RGB, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let reflected_direction = reflect_vec(&ray_in.direction.unit_vector(), &hit_record.normal)
            + self.fuzz * random_vec_in_unit_sphere();
        let reflected_ray = Ray::new(hit_record.point, reflected_direction, ray_in.time);
        if reflected_ray.direction.dot(&hit_record.normal) > 0.0 {
            return Some((reflected_ray, self.albedo));
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let new_direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                reflect_vec(&unit_direction, &hit_record.normal)
            } else {
                refract_vec(&unit_direction, &hit_record.normal, refraction_ratio)
            };

        Some((
            Ray::new(hit_record.point, new_direction, ray_in.time),
            RGB(1.0, 1.0, 1.0),
        ))
    }
}
