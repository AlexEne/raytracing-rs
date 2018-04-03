extern crate minifb;
extern crate rand;
mod vec3;
mod ray;

use rand::distributions::{IndependentSample, Range};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::{thread, time};
use vec3::Vec3;
use ray::Ray;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn color_at(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(0.0, 0.0, 1.0);
    }

    let t = 0.5 * (ray.dir().y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = vec3::dot(&r.dir(), &r.dir());
    let b = 2.0 * vec3::dot(&oc, &r.dir());
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; //RGBA
    let mut rng = rand::thread_rng();
    let color_range = Range::new(0, 255);
    // let num = rand::thread_rng().gen_range(0, 100);
    // println!("{}", num);
    let a: Vec3 = Vec3::new(0.0, 1.0, 2.0);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 1..HEIGHT {
            for x in 0..WIDTH {
                let u = (x as f32) / (WIDTH as f32);
                let v = (y as f32) / (HEIGHT as f32);
                let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
                let fcolor = color_at(&r);
                let color_r = (fcolor.r() * 255.99) as u32;
                let color_g = (fcolor.g() * 255.99) as u32;
                let color_b = (fcolor.b() * 255.99) as u32;
                buffer[((HEIGHT - y) * WIDTH + x) as usize] =
                    color_r << 16 | color_g << 8 | color_b;
            }
        }

        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();

        thread::sleep(time::Duration::from_millis(6000));
    }
}
