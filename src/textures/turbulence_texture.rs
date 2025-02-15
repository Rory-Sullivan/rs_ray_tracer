use crate::{colour::RGB, vec3d::Point3d};

use super::{perlin::Perlin, texture::Texture};

/// A Perlin noise texture using turbulence. Produces grey noise.
#[derive(Clone)]
pub struct TurbulenceTexture {
    noise: Perlin,
    scale: f64,
}

impl TurbulenceTexture {
    pub fn new(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}

impl Texture for TurbulenceTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3d) -> RGB {
        (self.noise.turbulence(self.scale * *p, None)) * RGB(1.0, 1.0, 1.0)
    }
}
