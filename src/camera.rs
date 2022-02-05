use crate::ray::Ray;
use glam::Vec3A;
use rand;
use rand::Rng;
use std;

pub struct Camera {
    lower_left_corner: Vec3A,
    horizontal: Vec3A,
    vertical: Vec3A,
    origin: Vec3A,
    lens_radius: f32,
    time_0: f32,
    time_1: f32,
    u: Vec3A,
    v: Vec3A,
    w: Vec3A,
}

pub const PI: f32 = std::f64::consts::PI as f32;

fn random_in_unit_disk() -> Vec3A {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3A::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0)
            - Vec3A::new(1.0, 1.0, 0.0);

        if Vec3A::dot(p, p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        up: Vec3A,
        v_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        t0: f32,
        t1: f32,
    ) -> Camera {
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = Vec3A::cross(up, w).normalize();
        let v = Vec3A::cross(w, u);

        Camera {
            lower_left_corner: look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            time_0: t0,
            time_1: t1,
            lens_radius: aperture / 2.0,
            w: w,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
