use std::ops;
use derive_more;
use derive_more::{Add, AddAssign, Display, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    DivAssign,
    MulAssign,
    Display,
)]
#[display("{x} {y} {z}")]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Scalar multiplication (Vec3 * f32)
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// Scalar multiplication (f32 * Vec3)
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self // Reuse the already defined Vec3 * f32 implementation
    }
}

// Scalar division (Vec3 / f32)
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        self * (1.0 / rhs)
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
    fn length_squared(self) -> f32 {
        self.dot(self)
    }

    fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

pub type Point3 = Vec3;