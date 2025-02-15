use crate::{colour::RGB, vec3d::Point3d};

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
