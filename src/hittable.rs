use crate::{material::Material, Point3d, Ray, Vec3d};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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

pub struct HittableList<'a> {
    items: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: Box<dyn Hittable + 'a>) {
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
}
