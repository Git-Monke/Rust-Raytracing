use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let center = self.center;
        let radius = self.radius;

        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(&oc);
        let c = oc.length_squared() - radius * radius;

        let discrim = half_b * half_b - a * c;

        if discrim < 0.0 {
            return false;
        }

        let sqrtd = discrim.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.time = root;
        hit_record.point = ray.at(root);
        hit_record.material = self.material;

        let normal = (hit_record.point - center) / radius;
        hit_record.set_normal(&ray, &normal);

        true
    }
}
