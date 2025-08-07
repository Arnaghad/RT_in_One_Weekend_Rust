use std::fs::File;
use crate::hittable::Hittable;
use crate::vec3::{Point3, Vec3};
use std::io::{self, Write};
use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pixel_samples_scale: f32,
    image_height: u32,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Self {
        let mut camera = Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            pixel_samples_scale: 0.0,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
        };

        camera.initialize();
        camera
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    pub fn render(&mut self, world: &Vec<Sphere>) -> io::Result<()> {
        self.initialize();
        let mut file = File::create("image.png")?;
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        for j in 0..self.image_height {
            eprint!("\rПромальовка рядків: {}/{}...", j + 1, self.image_height);
            io::stderr().flush()?;

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(r, self.max_depth, &world.as_slice());
                }
                // The line below for write_color also needs adjustment
                write_color(&mut file, &(pixel_color * self.pixel_samples_scale))?;
            }
        }

        eprintln!("\nDone!");
        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc + ((i as f32 + offset.x) * self.pixel_delta_u) + ((j as f32 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin; // Виправлений рядок
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        return Vec3::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5, 0.0);
    }
    fn ray_color(r: Ray, depth: u32, world:&impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        match world.hit(r, ..) {
            None => {
                let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a)
                    * Color {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }
                    + a * Color {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                }
            }
            Some(rec) => {
                let direction: Vec3 = rec.normal + Vec3::random_unit_vector();
                0.5 * Self::ray_color(Ray::new(rec.p, direction), depth - 1, world)
            }
        }
    }
}

struct Options {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }
}