pub mod colour;
pub mod hittable;
pub mod utilities;

mod camera;
mod ray;
mod sphere;
mod vec3d;

pub use vec3d::Vec3d;
pub type Point3d = Vec3d;
pub use camera::Camera;
pub use ray::Ray;
pub use sphere::Sphere;
