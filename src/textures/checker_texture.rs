use crate::{colour::RGB, vec3d::Point3d};

use super::texture::Texture;

/// A checkered texture, squares alternating between the odd and even colours.
#[derive(Debug, Clone, Copy)]
pub struct CheckerTexture<Tex0, Tex1>
where
    Tex0: Texture,
    Tex1: Texture,
{
    odd_colour: Tex0,
    even_colour: Tex1,
}

impl<Tex0, Tex1> CheckerTexture<Tex0, Tex1>
where
    Tex0: Texture,
    Tex1: Texture,
{
    pub fn new(odd_colour: Tex0, even_colour: Tex1) -> CheckerTexture<Tex0, Tex1> {
        Self {
            odd_colour,
            even_colour,
        }
    }
}

impl<Tex0, Tex1> Texture for CheckerTexture<Tex0, Tex1>
where
    Tex0: Texture,
    Tex1: Texture,
{
    fn value(&self, u: f64, v: f64, p: &Point3d) -> RGB {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd_colour.value(u, v, p);
        }
        self.even_colour.value(u, v, p)
    }
}
