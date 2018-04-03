extern crate minifb;
extern crate rand;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod world;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use minifb::{Key, Window, WindowOptions};
use std::{thread, time};
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::{HitRecord, Hittable};
use world::World;
use camera::Camera;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const SAMPLE_COUNT: usize = 100;

fn color_at(ray: &Ray, world: &World) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.0, 3.0, &mut rec) {
        let n = rec.normal;
        return 0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let t = 0.5 * (ray.dir().y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; //RGBA
    let mut rng = rand::thread_rng();
    // let color_range = Range::new(0, 255);
    // let num = rand::thread_rng().gen_range(0, 100);
    // println!("{}", num);
    let mut world = World::default();
    world.add_object(Box::new(Sphere::new(Vec3::new(0.4, 0.0, -1.0), 0.5)));
    world.add_object(Box::new(Sphere::new(Vec3::new(-0.6, 0.3, -2.0), 0.3)));
    let camera = Camera::new();

    let mut window = Window::new(
        "Raytracing on a plane - ESC to exit",
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

                //100 Samples per pixel
                let mut total = Vec3::default();
                for _ in 0..SAMPLE_COUNT {
                    let rx = rng.gen_range(0.0, 1.0);
                    let ry = rng.gen_range(0.0, 1.0);
                    let u = (x as f32 + rx) / (WIDTH as f32);
                    let v = (y as f32 + ry) / (HEIGHT as f32);
                    let r = camera.get_ray(u, v);
                    total = total + color_at(&r, &world);
                }
                let fcolor = total / (SAMPLE_COUNT as f32);
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

        thread::sleep(time::Duration::from_millis(1000));
    }
}
