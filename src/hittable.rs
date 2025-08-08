use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::collections::Bound;
use std::ops::RangeBounds;
use std::rc::Rc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

impl HitRecord {
    // A constructor that also determines if the ray hit the front or back face.
    pub fn new(t: f32, p: Point3, r: Ray, outward_normal: Vec3, mat: Rc<dyn Material>) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: impl RangeBounds<f32>) -> Option<HitRecord>;
}

// Implement Hittable for any slice of Hittable objects.
impl<'a, T> Hittable for &'a [T]
where
    &'a T: Hittable,
{
    fn hit(&self, r: Ray, ray_t: impl RangeBounds<f32>) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = match ray_t.start_bound() {
            Bound::Included(x) | Bound::Excluded(x) => *x,
            Bound::Unbounded => f32::INFINITY,
        };
        let ray_t_min = match ray_t.start_bound() {
            Bound::Included(x) | Bound::Excluded(x) => x.max(0.001),
            Bound::Unbounded => 0.001,
        };

        for object in self.iter() {
            if let Some(rec) = object.hit(r, ray_t_min..=closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}
