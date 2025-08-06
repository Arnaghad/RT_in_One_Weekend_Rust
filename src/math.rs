use std::f32::consts::PI;
use rand::Rng;
#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_f32(min: f32, max:f32) -> f32 {
    min + (max - min) * rand::random::<f32>()
}