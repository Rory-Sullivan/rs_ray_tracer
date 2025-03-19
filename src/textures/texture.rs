use crate::{colour::RGB, vec3d::Point3d};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3d) -> RGB;
}
