use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::utilities::clamp;

#[derive(Debug, Clone, Copy)]
pub struct RGB(pub f64, pub f64, pub f64);

impl RGB {
    pub fn to_integers(self, num_samples: usize) -> (usize, usize, usize) {
        // Divide by number of samples to average value
        let mut r = self.0 / num_samples as f64;
        let mut g = self.1 / num_samples as f64;
        let mut b = self.2 / num_samples as f64;

        // Take square root to gamma-correct for gamma = 2.0
        r = r.sqrt();
        g = g.sqrt();
        b = b.sqrt();

        // Convert ot int
        let ir = (256.0 * clamp(r, 0.0, 0.999)) as usize;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as usize;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as usize;

        (ir, ig, ib)
    }

    pub fn write_colour(self, num_samples: usize) -> String {
        let (ir, ig, ib) = self.to_integers(num_samples);
        format!("{ir} {ig} {ib}\n")
    }

    pub fn from_integers(r: usize, g: usize, b: usize) -> Self {
        assert!(
            r < 256 || g < 256 || b < 256,
            "RGB values out of range, must be less than 256; r: {r}, g: {g}, b: {b}"
        );

        RGB(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    pub fn from_hash(hash: &str) -> Self {
        assert!(hash.starts_with("#"));
        assert_eq!(hash.len(), 7);

        let r: usize = usize::from_str_radix(&hash[1..3], 16).unwrap();
        let g: usize = usize::from_str_radix(&hash[3..5], 16).unwrap();
        let b: usize = usize::from_str_radix(&hash[5..7], 16).unwrap();

        Self::from_integers(r, g, b)
    }
}

impl Add for RGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        RGB(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for RGB {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        RGB(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for RGB {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<RGB> for f64 {
    type Output = RGB;

    fn mul(self, rhs: RGB) -> Self::Output {
        RGB(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl MulAssign<f64> for RGB {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for RGB {
    type Output = RGB;

    fn div(self, rhs: f64) -> Self::Output {
        RGB(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for RGB {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Mul for RGB {
    type Output = RGB;

    fn mul(self, rhs: RGB) -> Self::Output {
        RGB(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
