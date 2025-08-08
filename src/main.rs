mod camera;
pub mod color;
pub mod hittable;
mod material;
pub mod math;
pub mod ray;
pub mod sphere;
pub mod vec3;

use crate::camera::{Camera, Options};
use crate::material::{Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3};
use std::rc::Rc;

fn main() {
    let mut world: Vec<Sphere> = Vec::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.push(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.push(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.push(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.push(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));
    let mut cam: Camera = Camera::new(Options {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        ..Default::default()
    });

    match cam.render(&world) {
        Ok(()) => println!("Rendering completed successfully"),
        Err(e) => eprintln!("Rendering failed: {}", e),
    }
}
