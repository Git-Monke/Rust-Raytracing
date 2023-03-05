use crate::{
    hittable::hit_record::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Clone, Copy)]
pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Lambertian { color }
    }

    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        let mut scatter_direction = record.normal + Vec3::random_unit_vec();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.point, scatter_direction);
        let attenuation = self.color;
        (true, scattered, attenuation)
    }
}
