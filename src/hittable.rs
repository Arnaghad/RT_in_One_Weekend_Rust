use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new (t: f32, p:Point3, r: Ray, outward_normal: Vec3) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            t,
            p,
            normal,
            front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f32, ray_tmax: f32) -> Option <HitRecord>;
}

pub struct HittableList<H: Hittable> {
    pub objects: Vec<H>,
}

impl<H: Hittable> HittableList<H> {
    pub fn new() -> HittableList<H> {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, object: H) {
        self.objects.push(object);
    }
    pub fn from_object(object : H) -> HittableList<H> {
        HittableList { objects: vec![object] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<H: Hittable> Hittable for HittableList<H> {
    fn hit(&self, r: Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }
        hit_anything
    }
}