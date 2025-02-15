use crate::{colour::RGB, vec3d::Point3d};

use super::texture::Texture;

/// A checkered texture, squares alternating between the odd and even colours.
#[derive(Clone)]
pub struct CheckerTexture {
    odd_colour: Box<dyn Texture + Sync>,
    even_colour: Box<dyn Texture + Sync>,
}

impl CheckerTexture {
    pub fn new(odd_colour: Box<dyn Texture + Sync>, even_colour: Box<dyn Texture + Sync>) -> Self {
        Self {
            odd_colour,
            even_colour,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3d) -> RGB {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd_colour.value(u, v, p);
        }
        self.even_colour.value(u, v, p)
    }
}
