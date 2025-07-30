pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod math;

use std::fs::File;
use std::io::{self, Write};
use crate::color::write_color;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::vec3::Point3;

/*fn hit_sphere (center: Point3, radius: f32,  r: Ray) -> f32 {
    let oc :Vec3 = center - r.origin();
    let a = r.direction().length_squared();
    let h = Vec3::dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}
*/

fn ray_color(r: Ray, world: impl Hittable) -> Color {
    match world.hit(r, 0.0, f32::INFINITY) {
        None => {
            let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color{x: 1.0, y: 1.0, z: 1.0} + a * Color{x: 0.5, y: 0.7, z: 1.0}
        }
        Some(rec) => {
            0.5 * (Color::new(1.0, 1.0, 1.0) + rec.normal)
        }
    }
}

fn main() -> std::io::Result<()> {
    // Розміри зображення
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut  image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = image_height.max(1);

    let mut world: Vec<Sphere> = Vec::new();

    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width: f32 = (image_width as f32 / image_height as f32) * viewport_height;
    let camera_center = Point3 {x:0.0, y:0.0, z:0.0};

    let viewport_u = Vec3 {x:viewport_width, y:0.0, z:0.0};
    let viewport_v = Vec3 {x:0.0, y:-viewport_height, z:0.0};

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center - Vec3 {x:0.0, y:0.0, z:focal_length} - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Створення файлу
    let mut file = File::create("image.ppm")?;

    // Запис заголовка PPM у файл
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    // Рендеринг пікселів
    for j in 0..image_height {
        // Повідомлення про прогрес, яке виводиться в консоль (у потік помилок).
        eprint!("\rПромальовка рядків: {}/{}...", j + 1, image_height);
        // Потрібно "скинути" буфер, щоб повідомлення з'явилося негайно.
        io::stderr().flush()?;

        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(r, world.as_slice());
            write_color(&mut file, &pixel_color)?;
        }
    }

    // Повідомлення про завершення
    eprintln!("\n✅ Готово! Зображення збережено як 'image.ppm'.");

    Ok(())
}