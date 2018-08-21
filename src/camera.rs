use ray::Ray;
use std;
use vec3;
use vec3::Vec3;

use rand;
use rand::Rng;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    time_0: f32,
    time_1: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

pub const PI: f32 = std::f64::consts::PI as f32;

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        
        if vec3::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
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
        let u = vec3::cross(&up, &w).normalize();
        let v = vec3::cross(&w, &u);

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
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
