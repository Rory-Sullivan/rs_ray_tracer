use crate::{
    colour::RGB,
    hittable::HitRecord,
    texture::{SolidColour, Texture},
    utilities::{random, random_unit_vec, random_vec_in_unit_sphere, reflect_vec, refract_vec},
    Point3d, Ray,
};

pub trait Material {
    /// Returns scattered ray and an attenuation colour
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)>;

    /// Return the colour of emitted light. Defaults to black for non-emissive
    /// materials.
    fn emitted(&self, _u: f64, _v: f64, _p: Point3d) -> RGB {
        RGB(0.0, 0.0, 0.0)
    }
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

/// Lambertian reflectance is the property that defines an ideal "matte" or
/// diffusely reflecting surface. This material is very similar to the Diffuse
/// material but it allows for generic textures to be passed in instead of a
/// solid colour.
#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture + Sync>) -> Self {
        Lambertian { albedo }
    }

    pub fn build_from_colour(colour: RGB) -> Self {
        Lambertian::new(Box::new(SolidColour::new(colour)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let mut scatter_direction = hit_record.normal + random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal
        }
        let ray_out = Ray::new(hit_record.point, scatter_direction, ray_in.time);
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);
        return Some((ray_out, attenuation));
    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    pub emit: Box<dyn Texture + Sync>,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture + Sync>) -> Self {
        DiffuseLight { emit }
    }

    pub fn build_from_colour(colour: RGB) -> Self {
        DiffuseLight::new(Box::new(SolidColour::new(colour)))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3d) -> RGB {
        self.emit.value(u, v, &p)
    }
}

/// An isotropic material that scatters rays in a random direction, used for
/// volumes like fog and smoke.
#[derive(Clone)]
pub struct Isotropic {
    pub albedo: Box<dyn Texture + Sync>,
}

impl Isotropic {
    pub fn new(albedo: Box<dyn Texture + Sync>) -> Self {
        Isotropic { albedo }
    }

    pub fn build_from_colour(colour: RGB) -> Self {
        Isotropic::new(Box::new(SolidColour::new(colour)))
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, RGB)> {
        let scattered = Ray::new(hit_record.point, random_vec_in_unit_sphere(), ray_in.time);
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);

        Some((scattered, attenuation))
    }
}
