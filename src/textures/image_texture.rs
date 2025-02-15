use crate::{
    colour::RGB,
    utilities::{clamp, read_image_file},
    vec3d::Point3d,
};

use super::texture::Texture;

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
