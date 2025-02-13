use crate::{
    material::Material,
    utilities::{degrees_to_radians, surrounding_box},
    BoundingBox, Point3d, Ray, Vec3d,
};

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
    pub material: &'a dyn Material,
    pub t: f64,
    pub u: f64, // value in [0, 1) representing the angle around the y-axis from x=-1 on unit sphere where hit occurs
    pub v: f64, // value in [0, 1) representing the angle from y=-1 to y=+1 on unit sphere where hit occurs
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3d,
        normal: Vec3d,
        material: &'a dyn Material,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> Self {
        Self {
            point,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn get_face_normal(ray: &Ray, outward_normal: Vec3d) -> (bool, Vec3d) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        (front_face, normal)
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

/// A translate trait to handle "moving" a hittable object. Does not actually
/// move the object but rather updates the hit function to "move" the ray before
/// passing it to the objects hit function.
#[derive(Clone)]
pub struct Translate {
    offset: Vec3d,
    object: Box<dyn Hittable + Sync>,
}

impl Translate {
    pub fn new(offset: Vec3d, object: Box<dyn Hittable + Sync>) -> Self {
        Self { offset, object }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        match self.object.hit(&moved_ray, t_min, t_max) {
            Some(hr) => {
                let (front_face, normal) = HitRecord::get_face_normal(&moved_ray, hr.normal);
                Some(HitRecord::new(
                    hr.point + self.offset,
                    normal,
                    hr.material,
                    hr.t,
                    hr.u,
                    hr.v,
                    front_face,
                ))
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<BoundingBox> {
        match self.object.bounding_box(time0, time1) {
            Some(bb) => Some(BoundingBox::new(bb.min + self.offset, bb.max + self.offset)),
            None => None,
        }
    }
}

/// A rotation instance to handle "rotating" a hittable object around the
/// x-axis. Does not actually rotate the object but rather updates the hit
/// function to "rotate" the ray before passing it to the objects hit function.
#[derive(Clone)]
pub struct RotateX {
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<BoundingBox>,
    object: Box<dyn Hittable + Sync>,
}

impl RotateX {
    pub fn new(angle: f64, object: Box<dyn Hittable + Sync>, t0: f64, t1: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        match object.bounding_box(t0, t1) {
            Some(obj_bb) => {
                let mut min = Point3d::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Point3d::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = (i as f64) * obj_bb.max.x + ((1 - i) as f64) * obj_bb.min.x;
                            let y = (j as f64) * obj_bb.max.y + ((1 - j) as f64) * obj_bb.min.y;
                            let z = (k as f64) * obj_bb.max.z + ((1 - k) as f64) * obj_bb.min.z;

                            let new_y = -sin_theta * z + cos_theta * y;
                            let new_z = cos_theta * z + sin_theta * y;

                            let temp_vec = Vec3d::new(x, new_y, new_z);
                            min.x = super::utilities::min(min.x, temp_vec.x);
                            min.y = super::utilities::min(min.y, temp_vec.y);
                            min.z = super::utilities::min(min.z, temp_vec.z);

                            max.x = super::utilities::max(max.x, temp_vec.x);
                            max.y = super::utilities::max(max.y, temp_vec.y);
                            max.z = super::utilities::max(max.z, temp_vec.z);
                        }
                    }
                }

                let bounding_box = BoundingBox::new(min, max);
                Self {
                    sin_theta,
                    cos_theta,
                    bounding_box: Some(bounding_box),
                    object,
                }
            }
            None => Self {
                sin_theta,
                cos_theta,
                bounding_box: None,
                object,
            },
        }
    }
}

impl Hittable for RotateX {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Vec3d::new(
            ray.origin.x,
            self.sin_theta * ray.origin.z + self.cos_theta * ray.origin.y,
            self.cos_theta * ray.origin.z - self.sin_theta * ray.origin.y,
        );
        let direction = Vec3d::new(
            ray.direction.x,
            self.sin_theta * ray.direction.z + self.cos_theta * ray.direction.y,
            self.cos_theta * ray.direction.z - self.sin_theta * ray.direction.y,
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        match self.object.hit(&rotated_ray, t_min, t_max) {
            Some(hr) => {
                let point = Point3d::new(
                    hr.point.x,
                    -self.sin_theta * hr.point.z + self.cos_theta * hr.point.y,
                    self.cos_theta * hr.point.z + self.sin_theta * hr.point.y,
                );
                let temp_normal = Point3d::new(
                    hr.normal.x,
                    -self.sin_theta * hr.normal.z + self.cos_theta * hr.normal.y,
                    self.cos_theta * hr.normal.z + self.sin_theta * hr.normal.y,
                );
                let (front_face, normal) = HitRecord::get_face_normal(&rotated_ray, temp_normal);

                Some(HitRecord::new(
                    point,
                    normal,
                    hr.material,
                    hr.t,
                    hr.u,
                    hr.v,
                    front_face,
                ))
            }
            None => None,
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        self.bounding_box
    }
}

/// A rotation instance to handle "rotating" a hittable object around the
/// y-axis. Does not actually rotate the object but rather updates the hit
/// function to "rotate" the ray before passing it to the objects hit function.
#[derive(Clone)]
pub struct RotateY {
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<BoundingBox>,
    object: Box<dyn Hittable + Sync>,
}

impl RotateY {
    pub fn new(angle: f64, object: Box<dyn Hittable + Sync>, t0: f64, t1: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        match object.bounding_box(t0, t1) {
            Some(obj_bb) => {
                let mut min = Point3d::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Point3d::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = (i as f64) * obj_bb.max.x + ((1 - i) as f64) * obj_bb.min.x;
                            let y = (j as f64) * obj_bb.max.y + ((1 - j) as f64) * obj_bb.min.y;
                            let z = (k as f64) * obj_bb.max.z + ((1 - k) as f64) * obj_bb.min.z;

                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;

                            let temp_vec = Vec3d::new(new_x, y, new_z);
                            min.x = super::utilities::min(min.x, temp_vec.x);
                            min.y = super::utilities::min(min.y, temp_vec.y);
                            min.z = super::utilities::min(min.z, temp_vec.z);

                            max.x = super::utilities::max(max.x, temp_vec.x);
                            max.y = super::utilities::max(max.y, temp_vec.y);
                            max.z = super::utilities::max(max.z, temp_vec.z);
                        }
                    }
                }

                let bounding_box = BoundingBox::new(min, max);
                Self {
                    sin_theta,
                    cos_theta,
                    bounding_box: Some(bounding_box),
                    object,
                }
            }
            None => Self {
                sin_theta,
                cos_theta,
                bounding_box: None,
                object,
            },
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Vec3d::new(
            self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
            ray.origin.y,
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
        );
        let direction = Vec3d::new(
            self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
            ray.direction.y,
            self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z,
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        match self.object.hit(&rotated_ray, t_min, t_max) {
            Some(hr) => {
                let point = Point3d::new(
                    self.cos_theta * hr.point.x + self.sin_theta * hr.point.z,
                    hr.point.y,
                    -self.sin_theta * hr.point.x + self.cos_theta * hr.point.z,
                );
                let temp_normal = Point3d::new(
                    self.cos_theta * hr.normal.x + self.sin_theta * hr.normal.z,
                    hr.normal.y,
                    -self.sin_theta * hr.normal.x + self.cos_theta * hr.normal.z,
                );
                let (front_face, normal) = HitRecord::get_face_normal(&rotated_ray, temp_normal);

                Some(HitRecord::new(
                    point,
                    normal,
                    hr.material,
                    hr.t,
                    hr.u,
                    hr.v,
                    front_face,
                ))
            }
            None => None,
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        self.bounding_box
    }
}

/// A rotation instance to handle "rotating" a hittable object around the
/// z-axis. Does not actually rotate the object but rather updates the hit
/// function to "rotate" the ray before passing it to the objects hit function.
#[derive(Clone)]
pub struct RotateZ {
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<BoundingBox>,
    object: Box<dyn Hittable + Sync>,
}

impl RotateZ {
    pub fn new(angle: f64, object: Box<dyn Hittable + Sync>, t0: f64, t1: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        match object.bounding_box(t0, t1) {
            Some(obj_bb) => {
                let mut min = Point3d::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Point3d::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = (i as f64) * obj_bb.max.x + ((1 - i) as f64) * obj_bb.min.x;
                            let y = (j as f64) * obj_bb.max.y + ((1 - j) as f64) * obj_bb.min.y;
                            let z = (k as f64) * obj_bb.max.z + ((1 - k) as f64) * obj_bb.min.z;

                            let new_x = -sin_theta * y + cos_theta * x;
                            let new_y = cos_theta * y + sin_theta * x;

                            let temp_vec = Vec3d::new(new_x, new_y, z);
                            min.x = super::utilities::min(min.x, temp_vec.x);
                            min.y = super::utilities::min(min.y, temp_vec.y);
                            min.z = super::utilities::min(min.z, temp_vec.z);

                            max.x = super::utilities::max(max.x, temp_vec.x);
                            max.y = super::utilities::max(max.y, temp_vec.y);
                            max.z = super::utilities::max(max.z, temp_vec.z);
                        }
                    }
                }

                let bounding_box = BoundingBox::new(min, max);
                Self {
                    sin_theta,
                    cos_theta,
                    bounding_box: Some(bounding_box),
                    object,
                }
            }
            None => Self {
                sin_theta,
                cos_theta,
                bounding_box: None,
                object,
            },
        }
    }
}

impl Hittable for RotateZ {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Vec3d::new(
            self.sin_theta * ray.origin.y + self.cos_theta * ray.origin.x,
            self.cos_theta * ray.origin.y - self.sin_theta * ray.origin.x,
            ray.origin.z,
        );
        let direction = Vec3d::new(
            self.sin_theta * ray.direction.y + self.cos_theta * ray.direction.x,
            self.cos_theta * ray.direction.y - self.sin_theta * ray.direction.x,
            ray.direction.z,
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        match self.object.hit(&rotated_ray, t_min, t_max) {
            Some(hr) => {
                let point = Point3d::new(
                    -self.sin_theta * hr.point.y + self.cos_theta * hr.point.x,
                    self.cos_theta * hr.point.y + self.sin_theta * hr.point.x,
                    hr.point.z,
                );
                let temp_normal = Point3d::new(
                    -self.sin_theta * hr.normal.y + self.cos_theta * hr.normal.x,
                    self.cos_theta * hr.normal.y + self.sin_theta * hr.normal.x,
                    hr.normal.z,
                );
                let (front_face, normal) = HitRecord::get_face_normal(&rotated_ray, temp_normal);

                Some(HitRecord::new(
                    point,
                    normal,
                    hr.material,
                    hr.t,
                    hr.u,
                    hr.v,
                    front_face,
                ))
            }
            None => None,
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<BoundingBox> {
        self.bounding_box
    }
}
