use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::RangeBounds;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for &Sphere {
    fn hit(&self, r: Ray, ray_t: impl RangeBounds<f32>) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if !ray_t.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal: Vec3 = (p - self.center) / self.radius;
        let mat = self.mat.clone();
        Some(HitRecord::new(t, p, r, outward_normal, mat))
    }
}
