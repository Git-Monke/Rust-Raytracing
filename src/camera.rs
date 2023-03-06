use crate::{degrees_to_radians, ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit();
        let u = vup.cross(&w);
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
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
