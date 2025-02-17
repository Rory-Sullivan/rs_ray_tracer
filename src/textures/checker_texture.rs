use crate::{colour::RGB, vec3d::Point3d};

use super::texture::Texture;

/// A checkered texture, squares alternating between the odd and even colours.
#[derive(Clone)]
pub struct CheckerTexture<TTexture0, TTexture1>
where
    TTexture0: Texture + Sync,
    TTexture1: Texture + Sync,
{
    odd_colour: TTexture0,
    even_colour: TTexture1,
}

impl<TTexture0, TTexture1> CheckerTexture<TTexture0, TTexture1>
where
    TTexture0: Texture + Sync,
    TTexture1: Texture + Sync,
{
    pub fn new(
        odd_colour: TTexture0,
        even_colour: TTexture1,
    ) -> CheckerTexture<TTexture0, TTexture1> {
        Self {
            odd_colour,
            even_colour,
        }
    }
}

impl<TTexture0, TTexture1> Texture for CheckerTexture<TTexture0, TTexture1>
where
    TTexture0: Texture + Clone + Sync,
    TTexture1: Texture + Clone + Sync,
{
    fn value(&self, u: f64, v: f64, p: &Point3d) -> RGB {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd_colour.value(u, v, p);
        }
        self.even_colour.value(u, v, p)
    }
}
