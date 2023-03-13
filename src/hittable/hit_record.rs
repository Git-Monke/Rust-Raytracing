use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use std::rc::Rc;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub time: f32,
    pub front_face: bool,
    pub material: Rc<Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            time: 0.0,
            front_face: true,
            material: Rc::new(Material::lambertian(Color::new(0.0, 0.0, 0.0))),
        }
    }

    pub fn set_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}
