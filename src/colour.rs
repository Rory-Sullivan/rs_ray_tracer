use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct RGB(pub f64, pub f64, pub f64);

impl RGB {
    pub fn write_colour(self) -> String {
        let ir = (255.999 * self.0) as usize;
        let ig = (255.999 * self.1) as usize;
        let ib = (255.999 * self.2) as usize;

        format!("{ir} {ig} {ib}\n")
    }
}

impl Add for RGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        RGB {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        }
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
        RGB {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        }
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
        RGB {
            0: self * rhs.0,
            1: self * rhs.1,
            2: self * rhs.2,
        }
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
        RGB {
            0: self.0 / rhs,
            1: self.1 / rhs,
            2: self.2 / rhs,
        }
    }
}

impl DivAssign<f64> for RGB {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}