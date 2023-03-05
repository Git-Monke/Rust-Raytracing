pub mod camera;
pub mod hittable;
pub mod ray;
pub mod vec3;

use camera::Camera;
use hittable::{hit_record::HitRecord, Hittable, HittableList};
use rand::Rng;
use ray::Ray;
use std::f32::consts::PI;
use std::f32::INFINITY;
use std::fs::File;
use std::io::{stdout, BufWriter, Error, Write};
use vec3::{Color, Vec3};

fn rand() -> f32 {
    rand::thread_rng().gen::<f32>()
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn write_color(
    writer: &mut BufWriter<File>,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> Result<(), Error> {
    let scale = 1.0 / samples_per_pixel as f32;
    let final_color = pixel_color * scale;
    writeln!(writer, "{}", final_color.clamped())
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut record = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, INFINITY, &mut record) {
        let target = record.point + record.normal + Vec3::random_unit_vec();

        return ray_color(
            &Ray::new(record.point, target - record.point),
            world,
            depth - 1,
        ) * 0.5;
    }

    let unit = ray.direction.unit();
    let t = 0.5 * (1.0 + unit.y);

    Color::from_percent(1.0, 1.0, 1.0) * (1.0 - t) + Color::from_percent(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Error> {
    // Program Configuration
    let aspect_ratio: f32 = 16.0 / 9.0;

    // Image Settings
    let image_width: i32 = 500;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let depth = 10;

    // File formatting
    let output = File::create("output.ppm")?;
    let mut writer = BufWriter::new(output);

    let image_format = format!("P3\n{} {}\n255", image_width, image_height);
    writeln!(writer, "{image_format}")?;

    // Camera
    let camera = Camera::new(16.0 / 9.0, 2.0, Vec3::new(0.0, 0.0, 0.0));

    // World generation
    let mut world = HittableList::new();
    world.add(Hittable::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Hittable::sphere(Vec3::new(2.0, 0.0, -2.0), 0.5));
    world.add(Hittable::sphere(Vec3::new(-2.0, 1.0, -2.0), 0.5));
    world.add(Hittable::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for i in (0..image_height).rev() {
        print!(
            "\r{}% Finished rendering",
            100 - ((i as f32 / image_height as f32) * 100.0) as i32
        );
        stdout().flush().unwrap();

        for j in 0..image_width {
            let mut color = Color::new(0.0, 0.0, 0.0);

            let j = j as f32;
            let i = i as f32;
            let image_width = image_width as f32;
            let image_height = image_height as f32;

            for _ in 0..samples_per_pixel {
                let u = (j + rand()) / image_width;
                let v = (i + rand()) / image_height;

                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, depth);
            }

            write_color(&mut writer, color, samples_per_pixel)?;
        }
    }

    writer.flush().unwrap();

    Ok(())
}
