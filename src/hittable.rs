pub mod hit_record;
pub mod sphere;

use hit_record::HitRecord;
use sphere::Sphere;

use crate::{material::Material, ray::Ray, vec3::Vec3};

pub enum Hittable {
    Sphere(Sphere),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere(object) => object.hit(&ray, t_min, t_max, hit_record),
        }
    }

    pub fn sphere(center: Vec3, radius: f32, material: Material) -> Hittable {
        Hittable::Sphere(Sphere::new(center, radius, material))
    }
}

pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut hit = false;
        let mut smallest_t = t_max;

        for object in self.objects.iter() {
            if object.hit(&ray, t_min, smallest_t, hit_record) {
                hit = true;
                smallest_t = hit_record.time;
            }
        }

        hit
    }
}
