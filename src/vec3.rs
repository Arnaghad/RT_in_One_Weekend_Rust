use crate::math::random_f32;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops;
use std::ops::RangeInclusive;

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// Scalar multiplication (f32 * Vec3)
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        Self::new(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        )
    }
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Self::new(
            random_f32(min, max),
            random_f32(min, max),
            random_f32(min, max),
        )
    }
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if lensq <= 1.0 && lensq > 1e-160 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn to_bytes(&self) -> [u8; 3] {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;

        r = crate::color::linear_to_gamma(r);
        g = crate::color::linear_to_gamma(g);
        b = crate::color::linear_to_gamma(b);

        let intensity: RangeInclusive<f32> = 0.00..=0.99;

        let rbyte = (256.000 * r.clamp(*intensity.start(), *intensity.end())) as u8;
        let gbyte = (256.000 * g.clamp(*intensity.start(), *intensity.end())) as u8;
        let bbyte = (256.000 * b.clamp(*intensity.start(), *intensity.end())) as u8;
        [rbyte, gbyte, bbyte]
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }
}
