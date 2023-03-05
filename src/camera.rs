use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, viewport_height: f32, origin: Vec3) -> Self {
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

// Camera Settings
// let viewport_height = 2.0;
// let viewport_width = viewport_height * aspect_ratio;
// let focal_length = 1.0;

// let origin = Vec3::new(0.0, 0.0, 0.0);
// let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
// let vertical = Vec3::new(0.0, viewport_height, 0.0);
// let lower_left_corner =
//     origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
