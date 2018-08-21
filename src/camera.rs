use ray::Ray;
use std;
use vec3::Vec3;
use vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

pub const PI: f32 = std::f64::consts::PI as f32;

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, v_fov: f32, aspect: f32) -> Camera {
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let mut w = look_from - look_at;
        w.normalize();
        let mut u = vec3::cross(&up, &w);
        u.normalize();
        let v = vec3::cross(&w, &u);

        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
