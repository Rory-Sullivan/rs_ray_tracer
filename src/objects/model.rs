use std::fs::File;
use std::io::{self, BufRead};

use crate::bvh::bvh::BvhMetrics;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{hit_record::HitRecord, hittable::Hittable};
use crate::{
    bvh::{bounding_box::BoundingBox, bvh::Bvh},
    materials::Material,
    ray::Ray,
    vec3d::Vec3d,
};

use super::triangle::Triangle;

/// Struct for storing data related to a 3D model.
#[derive(Clone)]
pub struct Model<'a> {
    bvh: Bvh<'a>,
}

impl<'a> Model<'a> {
    pub fn new(bvh: Bvh<'a>) -> Self {
        Self { bvh }
    }

    pub fn build<TMaterial>(file_name: &str, material: TMaterial) -> (Model<'a>, BvhMetrics)
    where
        TMaterial: Material + Clone + 'a,
    {
        let time0 = 0.0;
        let time1 = 0.0;

        let triangles = read_ply_file(file_name);

        let mut hittable_triangles = HittableList::new(time0, time1);
        for tri in triangles {
            let triangle = Triangle::new(tri.0, tri.1, tri.2, material.clone());
            hittable_triangles.add(Box::new(triangle));
        }

        let (bvh, bvh_metrics) = Bvh::build(hittable_triangles, time0, time1);

        (Self::new(bvh), bvh_metrics)
    }
}

impl Hittable for Model<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bvh.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox> {
        self.bvh.bounding_box(time0, time1)
    }
}

fn read_ply_file<'a>(file_name: &str) -> Vec<(Vec3d, Vec3d, Vec3d)> {
    let file = File::open(file_name).expect("Error opening file");
    let mut lines = io::BufReader::new(file).lines();

    // Read header
    let mut num_vertices: usize = 0;
    let mut num_faces: usize = 0;

    // Iterate over file till the 'end header' line extracting necessary
    // information
    loop {
        let line = lines.next().unwrap().unwrap();
        match line.as_str() {
            line if line.starts_with("element vertex ") => {
                num_vertices = line[15..].parse().unwrap();
            }
            line if line.starts_with("element face ") => {
                num_faces = line[13..].parse().unwrap();
            }
            "end_header" => {
                break;
            }
            _ => {} // Ignore all other lines of header
        }
    }

    // Check we read necessary data from header
    assert_ne!(num_vertices, 0);
    assert_ne!(num_faces, 0);

    // Read vertices
    let mut vertices = Vec::<Vec3d>::new();
    for _ in 0..num_vertices {
        // 0 0 0
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<f64> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        assert_eq!(parts.len(), 3);
        vertices.push(Vec3d::new(parts[0], parts[1], parts[2]));
    }

    // Read faces
    let mut triangles = Vec::<(Vec3d, Vec3d, Vec3d)>::new();
    for _ in 0..num_faces {
        // 3 0 1 3
        let line = lines.next().unwrap().unwrap();
        assert!(line.starts_with("3 "));
        let parts: Vec<usize> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        assert_eq!(parts.len(), 4);
        triangles.push((vertices[parts[1]], vertices[parts[2]], vertices[parts[3]]));
    }

    triangles
}
