use std::f32::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}
