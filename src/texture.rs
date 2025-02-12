use crate::{
    colour::RGB,
    perlin::Perlin,
    utilities::{clamp, read_image_file},
    Point3d,
};

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

/// Create a texture from an image file.
#[derive(Clone)]
pub struct ImageTexture {
    width: usize,
    height: usize,
    pixels: Vec<RGB>,
}

impl ImageTexture {
    pub fn new(width: usize, height: usize, pixels: Vec<RGB>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn build(file_name: &str) -> Self {
        let (width, height, pixels) = read_image_file(file_name);
        Self::new(width, height, pixels)
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3d) -> RGB {
        // Clamp input texture coordinates
        let image_u = clamp(u, 0.0, 1.0);
        let image_v = 1.0 - clamp(v, 0.0, 1.0); // Flip v to image coordinates

        let mut i = (image_u * (self.width as f64)) as usize;
        let mut j = (image_v * (self.height as f64)) as usize;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let pixel = self.pixels[(j * self.width) + i];
        pixel
    }
}
