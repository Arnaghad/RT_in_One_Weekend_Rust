use crate::vec3::Vec3;
use std::io::Write;
use std::ops::{RangeBounds, RangeInclusive};

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: &Color) -> std::io::Result<()> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let intensity: RangeInclusive<f32> = 0.00..=0.99;

    let rbyte = (256.000 * r.clamp(*intensity.start(), *intensity.end())) as u8;
    let gbyte = (256.000 * g.clamp(*intensity.start(), *intensity.end())) as u8;
    let bbyte = (256.000 * b.clamp(*intensity.start(), *intensity.end())) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;
    Ok(())
}
