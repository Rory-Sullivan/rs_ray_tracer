use crate::{colour::RGB, vec3d::Point3d};

use super::texture::Texture;

#[derive(Clone, Copy)]
pub struct SolidColour {
    colour: RGB,
}

impl SolidColour {
    pub fn new(colour: RGB) -> Self {
        Self { colour }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: &Point3d) -> RGB {
        self.colour
    }
}
