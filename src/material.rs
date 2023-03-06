mod dialectric;
mod lambertian;
mod metal;

use dialectric::Dialectric;
use lambertian::Lambertian;
use metal::Metal;

use crate::{hittable::hit_record::HitRecord, ray::Ray, vec3::Color};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialectric(Dialectric),
}

impl Material {
    pub fn lambertian(color: Color) -> Material {
        Material::Lambertian(Lambertian::new(color))
    }

    pub fn metal(color: Color, fuzz: f32) -> Material {
        Material::Metal(Metal::new(color, fuzz))
    }

    pub fn dialectric(refraction: f32) -> Material {
        Material::Dialectric(Dialectric::new(refraction))
    }

    // Scatter returns if the light was reflected as the first parameter
    // If it was, then it will return the new ray as the second parameter and the color it hit as the third
    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        match self {
            Material::Lambertian(material) => material.scatter(ray, record),
            Material::Metal(material) => material.scatter(ray, record),
            Material::Dialectric(material) => material.scatter(ray, record),
        }
    }
}
