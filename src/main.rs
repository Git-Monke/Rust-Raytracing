pub mod hittable;
pub mod ray;
pub mod vec3;

use hittable::{hit_record::HitRecord, Hittable, HittableList};
use ray::Ray;
use std::f32::consts::PI;
use std::f32::INFINITY;
use std::fs::File;
use std::io::{stdout, BufWriter, Error, Write};
use vec3::{Color, Vec3};

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut record = HitRecord::new();

    if world.hit(ray, 0.0, INFINITY, &mut record) {
        let normal = record.normal;
        return (Color::from_percent(normal.x, normal.y, normal.z)
            + Color::from_percent(1.0, 1.0, 1.0))
            * 0.5;
    }

    let unit = ray.direction.unit();
    let t = 0.5 * (1.0 + unit.y);

    Color::from_percent(1.0, 1.0, 1.0) * (1.0 - t) + Color::from_percent(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Error> {
    // Program Configuration
    let aspect_ratio: f32 = 16.0 / 9.0;

    // Image Formatting
    let image_width: i32 = 1000;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    // Camera Settings

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // File formatting
    let output = File::create("output.ppm")?;
    let mut writer = BufWriter::new(output);

    let image_format = format!("P3\n{} {}\n255", image_width, image_height);
    writeln!(writer, "{image_format}")?;

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
            let u = j as f32 / image_width as f32;
            let v = i as f32 / image_height as f32;

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            writeln!(writer, "{}", ray_color(&ray, &world))?;
        }
    }

    writer.flush().unwrap();

    Ok(())
}
