use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A vector in 3-dimensional space.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x, y, z }
    }

    pub fn len_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Returns the normalised unit vector
    pub fn unit_vector(&self) -> Vec3d {
        *self / self.len()
    }

    pub fn dot(&self, rhs: &Vec3d) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Vec3d) -> Vec3d {
        Vec3d {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    /// Returns the value of a given axis;
    ///
    /// - 0 is x-axis
    /// - 1 is y-axis
    /// - 2 is z-axis
    ///
    /// Any other value will cause the function to panic.
    pub fn get_axis(&self, axis: usize) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Index out of bounds; index: {axis}"),
        }
    }

    /// Returns true if the vector is close to zero in all directions
    pub fn near_zero(&self) -> bool {
        const THRESHOLD: f64 = 1e-8;
        self.x.abs() < THRESHOLD && self.y.abs() < THRESHOLD && self.z.abs() < THRESHOLD
    }

    /// Scales the vector by the given amount for each axis. Returns a scaled
    /// copy.
    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        Self::new(self.x * x, self.y * y, self.z * z)
    }
}

impl Add for Vec3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3d> for f64 {
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Self::Output {
        Vec3d {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3d {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

/// A point in 3-dimensional space.
pub type Point3d = Vec3d;
