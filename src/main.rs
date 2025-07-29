mod vec3;
mod color;
mod ray;

fn ray_color(r: Ray) -> Color {
    Color { x: 0.0, y: 0.0, z: 0.0 }
}

// Імпортуємо необхідні компоненти.
use std::fs::File;
use std::io::{self, Write};
use crate::color::write_color; // Імпортуємо функцію з модуля Color
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() -> std::io::Result<()> {
    // Розміри зображення
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut  image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = image_height.max(1);

    let viewport_height = 2.0;
    let viewport_width = (image_width / image_height) as f64 * viewport_height;

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
            let pixel_color = Color {
                x: i as f32 / (image_width - 1) as f32,
                y: j as f32 / (image_height - 1) as f32,
                z: 0.0,
            };

            write_color(&mut file, &pixel_color)?;
        }
    }

    // Повідомлення про завершення
    eprintln!("\n✅ Готово! Зображення збережено як 'image.ppm'.");

    Ok(())
}