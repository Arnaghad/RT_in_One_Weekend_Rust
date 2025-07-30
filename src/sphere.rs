use std::hash::RandomState;
use std::num::FpCategory::Nan;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere{center, radius: radius.max(0.0)}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - (self.radius * self.radius) as f32;

        let discriminant = (h * h) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal: Vec3 = (p - self.center) / self.radius;
        Some(HitRecord::new(t, p, r, outward_normal))
    }
}