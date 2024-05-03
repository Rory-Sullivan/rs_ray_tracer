pub mod colour;
pub mod hittable;
pub mod material;
pub mod render;
pub mod utilities;

mod camera;
mod ray;
mod resolution;
mod sphere;
mod vec3d;

pub use camera::Camera;
pub use ray::Ray;
pub use resolution::Resolution;
pub use sphere::Sphere;
pub use vec3d::Vec3d;

pub type Point3d = Vec3d;
