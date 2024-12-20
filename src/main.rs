#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod vec3;

use camera::Camera;
use hittable::{hit_record::HitRecord, Hittable, HittableList};
use material::Material;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use std::f32::consts::PI;
use std::f32::INFINITY;
use std::fs::File;
use std::io::{stdout, BufWriter, Error, Write};
// use std::thread;
// use std::time::{Duration, Instant};
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
    samples_per_pixel: usize,
) -> Result<(), Error> {
    let scale = 1.0 / samples_per_pixel as f32;
    let final_color = pixel_color * scale;
    writeln!(writer, "{}", final_color.as_color_triplet())
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut record = HitRecord::new();

    if depth <= 0 {
        return Color::new(1.0, 1.0, 1.0);
    }

    if world.hit(ray, 0.001, INFINITY, &mut record) {
        let (was_scattered, scattered_ray, color) = record.material.scatter(ray, &record);

        if was_scattered {
            return color * ray_color(&scattered_ray, world, depth - 1);
        }

        return Color::new(1.0, 1.0, 1.0);
    }

    let unit = ray.direction.unit();
    let t = 0.5 * (1.0 + unit.y);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Error> {
    // Program Configuration
    let aspect_ratio: f32 = 16.0 / 9.0;

    // Image Settings
    const SAMPLES_PER_PIXEL: usize = 500;
    let image_width: i32 = 250;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let depth = 10;

    // File formatting
    let output = File::create("output.ppm")?;
    let mut writer = BufWriter::new(output);

    let image_format = format!("P3\n{} {}\n255", image_width, image_height);
    writeln!(writer, "{image_format}")?;

    // Camera
    let look_from = Vec3::new(15.0, 6.0, 15.0);
    let look_at = Vec3::new(12.0, 1.0, 12.0);
    let up_vector = Vec3::new(0.0, 1.0, 0.0);

    let focus_dist = (look_from - look_at).length();
    let aperture = 0.5;

    let camera = Camera::new(
        16.0 / 9.0,
        look_from,
        look_at,
        up_vector,
        80.0,
        aperture,
        focus_dist,
    );

    // World generation
    let mut world = HittableList::new();

    world.add(Hittable::sphere(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian(Color::new(0.5, 0.5, 0.5)),
    ));

    let sphere_count = 10;

    for i in 0..sphere_count {
        for j in 0..sphere_count {
            let material_num = rand();
            let material;

            if material_num > 0.66 {
                material = Material::lambertian(Color::random());
            } else if material_num > 0.33 {
                material = Material::metal(Color::random(), rand());
            } else {
                material = Material::dialectric(1.0 + rand());
            }

            world.add(Hittable::sphere(
                Vec3::new(i as f32 * 2.0, 1.0, j as f32 * 2.0),
                1.0,
                material,
            ));
        }
    }

    for i in (0..image_height).rev() {
        print!(
            "\r{}% Finished rendering",
            100 - ((i as f32 / image_height as f32) * 100.0) as i32
        );
        stdout().flush().unwrap();

        for j in 0..image_width {
            // let mut sum = Color::new(0.0, 0.0, 0.0);

            let j = j as f32;
            let i = i as f32;
            let image_width = image_width as f32;
            let image_height = image_height as f32;

            // for _ in 0..samples_per_pixel {
            //     let u = (j + rand()) / image_width;
            //     let v = (i + rand()) / image_height;

            //     let ray = camera.get_ray(u, v);
            //     sum += ray_color(&ray, &world, depth);
            // }

            let mut colors = [None; SAMPLES_PER_PIXEL];

            colors.par_iter_mut().for_each(|p| {
                let u = (j + rand()) / image_width;
                let v = (i + rand()) / image_height;

                let ray = camera.get_ray(u, v);
                *p = Some(ray_color(&ray, &world, depth));
            });

            let mut sum = Vec3::new(0.0, 0.0, 0.0);

            for color in colors.iter() {
                sum += color.unwrap();
            }

            write_color(&mut writer, sum, SAMPLES_PER_PIXEL)?;
        }
    }

    writer.flush().unwrap();

    Ok(())
}
