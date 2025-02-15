use crate::{colour::RGB, vec3d::Point3d};

use super::{perlin::Perlin, texture::Texture};

/// A Perlin noise texture. Produces grey noise.
#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3d) -> RGB {
        (1.0 + self.noise.noise(self.scale * *p)) * 0.5 * RGB(1.0, 1.0, 1.0)
    }
}
