use crate::vec3::Vec3;
use std::io::Write;
use std::ops::{RangeBounds, RangeInclusive};

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: &Color) -> std::io::Result<()> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);
    
    let intensity: RangeInclusive<f32> = 0.00..=0.99;

    let rbyte = (256.000 * r.clamp(*intensity.start(), *intensity.end())) as u8;
    let gbyte = (256.000 * g.clamp(*intensity.start(), *intensity.end())) as u8;
    let bbyte = (256.000 * b.clamp(*intensity.start(), *intensity.end())) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;
    Ok(())
}

#[inline]
fn linear_to_gamma(linear_component: f32) -> f32 {
    if (linear_component > 0.0) {
        return f32::sqrt(linear_component)
    }
    0.0
}