pub struct Resolution {
    pub image_width: usize,
    pub image_height: usize,
    pub num_samples: usize,
    pub max_depth: usize,
}

impl Resolution {
    pub fn new(
        image_width: usize,
        image_height: usize,
        num_samples: usize,
        max_depth: usize,
    ) -> Resolution {
        Resolution {
            image_width,
            image_height,
            num_samples,
            max_depth,
        }
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        (self.image_width as f64) / (self.image_height as f64)
    }
}
