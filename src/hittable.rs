use crate::{material::Material, utilities::surrounding_box, BoundingBox, Point3d, Ray, Vec3d};

/// Trait for all objects that can be hit by a ray.
pub trait Hittable: DynClone {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox>;
}

/// Helper trait to make Box<dyn Hittable + Sync + '_> types clone-able. This is
/// necessary for clones that happen in the Bvh::build function.
///
/// Read more here https://quinedot.github.io/rust-learning/dyn-trait-clone.html
pub trait DynClone {
    fn dyn_clone<'a>(&self) -> Box<dyn Hittable + Sync + 'a>
    where
        Self: 'a;
}

impl<T: Clone + Hittable + Sync> DynClone for T {
    fn dyn_clone<'a>(&self) -> Box<dyn Hittable + Sync + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable + Sync + '_> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}

pub struct HitRecord<'a> {
    pub point: Point3d,
    pub normal: Vec3d,
    pub material: Box<dyn Material + 'a>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn new(
        point: Point3d,
        normal: Vec3d,
        material: Box<dyn Material + '_>,
        t: f64,
        front_face: bool,
    ) -> HitRecord<'_> {
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

#[derive(Clone)]
pub struct HittableList<'a> {
    pub items: Vec<Box<dyn Hittable + Sync + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: Box<dyn Hittable + Sync + 'a>) {
        self.items.push(item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for item in self.items.iter() {
            match item.hit(ray, t_min, closest_so_far) {
                Some(hr) => {
                    closest_so_far = hr.t;
                    hit_record = Some(hr);
                }
                None => {}
            }
        }

        hit_record
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox> {
        if self.items.is_empty() {
            return None;
        }

        let mut temp_box = self.items[0].bounding_box(time0, time1);
        let mut output_box = match temp_box {
            None => return None,
            Some(x) => x,
        };

        for object in self.items[1..].iter() {
            temp_box = object.bounding_box(time0, time1);
            if temp_box.is_none() {
                return None;
            }
            output_box = surrounding_box(output_box, temp_box.unwrap());
        }

        Some(output_box)
    }
}
