pub mod box_obj;
pub mod colour;
pub mod hittable;
pub mod material;
pub mod perlin;
pub mod rectangle;
pub mod render;
pub mod texture;
pub mod utilities;

mod bounding_box;
mod bvh;
mod camera;
mod moving_sphere;
mod ray;
mod resolution;
mod sphere;
mod vec3d;

pub use bounding_box::BoundingBox;
pub use bvh::Bvh;
pub use camera::Camera;
pub use moving_sphere::MovingSphere;
pub use ray::Ray;
pub use resolution::Resolution;
pub use sphere::Sphere;
pub use vec3d::Vec3d;

pub type Point3d = Vec3d;
