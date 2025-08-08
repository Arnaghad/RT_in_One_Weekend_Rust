pub mod color;
pub mod hittable;
pub mod math;
pub mod ray;
pub mod sphere;
pub mod vec3;
mod camera;
mod material;

use std::io::Error;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use crate::camera::Camera;

fn main() {
    let mut world: Vec<Sphere> = Vec::new();
    world.push(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0));
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 100, 50);

    match cam.render(&world) {
        Ok(()) => println!("Rendering completed successfully"),
        Err(e) => eprintln!("Rendering failed: {}", e),
    }
}
