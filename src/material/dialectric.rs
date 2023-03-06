use rand::Rng;

use crate::{
    hittable::hit_record::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Clone, Copy)]
pub struct Dialectric {
    refraction: f32,
}

fn rand() -> f32 {
    rand::thread_rng().gen::<f32>()
}

impl Dialectric {
    pub fn new(refraction: f32) -> Self {
        Dialectric { refraction }
    }

    fn reflectance(cosine: f32, refraction: f32) -> f32 {
        let mut r0 = (1.0 - refraction) / (1.0 + refraction);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = match record.front_face {
            true => 1.0 / self.refraction,
            false => self.refraction,
        };

        let unit_direction = ray.direction.unit();
        let cos_theta = ((-unit_direction).dot(&record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || Dialectric::reflectance(cos_theta, refraction_ratio) > rand() {
            direction = Vec3::reflect(unit_direction, record.normal);
        } else {
            direction = Vec3::refract(unit_direction, record.normal, refraction_ratio);
        }

        let scattered = Ray::new(record.point, direction);

        (true, scattered, attenuation)
    }
}
