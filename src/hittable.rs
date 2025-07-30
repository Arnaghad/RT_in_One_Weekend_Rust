use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    // A constructor that also determines if the ray hit the front or back face.
    pub fn new(t: f32, p: Point3, r: Ray, outward_normal: Vec3) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self { p, normal, t, front_face }
    }
}

pub trait Hittable {
    fn hit(self, r: Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}

// Implement Hittable for any slice of Hittable objects.
impl<'a, T> Hittable for &'a [T]
where &'a T: Hittable {
    fn hit(self, r: Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_tmax;

        for object in self.iter() {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}