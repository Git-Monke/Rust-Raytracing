use crate::{
    hittable::hit_record::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Clone, Copy)]
pub struct Metal {
    color: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(color: Color, fuzz: f32) -> Self {
        Metal { color, fuzz }
    }

    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        let reflected = Vec3::reflect(ray.direction.unit(), record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + Vec3::rand_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self.color;
        let was_scattered = scattered.direction.dot(&record.normal) > 0.0;
        (was_scattered, scattered, attenuation)
    }
}
