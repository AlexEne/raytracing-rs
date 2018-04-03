extern crate minifb;
extern crate rand;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod world;
mod material;

use rand::Rng;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{thread, time};
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::{HitRecord, Hittable};
use world::World;
use camera::Camera;
use material::Material;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const SAMPLE_COUNT: usize = 50;

fn color_at(ray: &Ray, world: &World, depth: u32) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.0001, std::f32::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();
        let rec_c = HitRecord {
            p: rec.p,
            normal: rec.normal,
            t: rec.t,
            material: None,
        };
        if let Some(material) = rec.material {
            if depth < 50
                && material::scatter(&material, ray, &rec_c, &mut attenuation, &mut scattered)
            {
                return attenuation * color_at(&scattered, world, depth + 1);
            } else {
                return Vec3::default();
            }
        } else {
            panic!("No material wtf!");
        }
    } else {
        let t = 0.5 * (ray.dir().y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn generate_scene(buffer: &mut Vec<u32>) {
    let mut rng = rand::thread_rng();
    let mut world = World::default();
    world.add_object(Box::new(Sphere::new(
        Vec3::new(0.2, 0.0, -1.0),
        0.5,
        // Material::Lambertian {
        //     albedo: Vec3::new(0.8, 0.3, 0.3),
        // },
        Material::Metal {
            albedo: Vec3::new(0.8, 0.8, 0.8),
            fuzz: 0.1,
        }
    )));
    world.add_object(Box::new(Sphere::new(
        Vec3::new(-0.8, -0.2, -2.0),
        0.3,
        Material::Lambertian {
            albedo: Vec3::new(0.2, 0.4, 0.2),
        },
    )));
    world.add_object(Box::new(Sphere::new(
        Vec3::new(2.5, 0.0, -2.3),
        0.5,
        Material::Metal {
            albedo: Vec3::new(0.2, 0.2, 0.3),
            fuzz: 0.4,
        },
    )));
    world.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.2),
        },
    )));

    for _ in 0..20 {
        let x = rng.gen_range(-5.0, 5.0);
        let z = rng.gen_range(-2.0, 0.5);
        let r = rng.gen_range(0.0, 1.0);
        let g = rng.gen_range(0.0, 1.0);
        let b = rng.gen_range(0.0, 1.0);
        let radius = rng.gen_range(0.05, 0.2);
        world.add_object(Box::new(Sphere::new(
            Vec3::new(x, -0.3, z),
            radius,
            Material::Lambertian {
                albedo: Vec3::new(r, g, b),
            },
        )));

        let x = rng.gen_range(-5.0, 5.0);
        let z = rng.gen_range(-2.0, -0.4);
        let r = rng.gen_range(0.0, 1.0);
        let g = rng.gen_range(0.0, 1.0);
        let b = rng.gen_range(0.0, 1.0);
        let radius = rng.gen_range(0.05, 0.2);
        let fuzz = rng.gen_range(0.0, 0.7);
        world.add_object(Box::new(Sphere::new(
            Vec3::new(x, -0.3, z),
            radius,
            Material::Metal {
                albedo: Vec3::new(r, g, b),
                fuzz: fuzz,
            },
        )));
    }

    let camera = Camera::new();

    //Render the scene
    for y in 1..HEIGHT {
        for x in 0..WIDTH {
            let mut total = Vec3::default();
            for _ in 0..SAMPLE_COUNT {
                let rx = rng.gen_range(0.0, 1.0);
                let ry = rng.gen_range(0.0, 1.0);
                let u = (x as f32 + rx) / (WIDTH as f32);
                let v = (y as f32 + ry) / (HEIGHT as f32);
                let r = camera.get_ray(u, v);
                total = total + color_at(&r, &world, 0);
            }
            let fcolor = total / (SAMPLE_COUNT as f32);
            let fcolor = Vec3::new(fcolor.x().sqrt(), fcolor.y().sqrt(), fcolor.z().sqrt());
            let color_r = (fcolor.r() * 255.99) as u32;
            let color_g = (fcolor.g() * 255.99) as u32;
            let color_b = (fcolor.b() * 255.99) as u32;
            buffer[((HEIGHT - y) * WIDTH + x) as usize] = color_r << 16 | color_g << 8 | color_b;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; //R..G..B..R..G..B
    
    let mut window = Window::new(
        "Raytracing on a plane - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    generate_scene(&mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            generate_scene(&mut buffer);
        }

        window.update_with_buffer(&buffer).unwrap();

        thread::sleep(time::Duration::from_millis(33));
    }
}
