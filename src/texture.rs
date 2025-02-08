use crate::{colour::RGB, perlin::Perlin, Point3d};

pub trait Texture: DynClone {
    fn value(&self, u: f64, v: f64, p: &Point3d) -> RGB;
}

/// Helper trait to make Box<dyn Texture + Sync + 'a> types clone-able. This is
/// necessary because textures are cloned into objects.
///
/// Read more here https://quinedot.github.io/rust-learning/dyn-trait-clone.html
pub trait DynClone {
    fn dyn_clone<'a>(&self) -> Box<dyn Texture + Sync + 'a>
    where
        Self: 'a;
}

impl<T: Clone + Texture + Sync> DynClone for T {
    fn dyn_clone<'a>(&self) -> Box<dyn Texture + Sync + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Texture + Sync + '_> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}

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
