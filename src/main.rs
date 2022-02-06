extern crate minifb;
extern crate rand;
extern crate rayon;

use bvh::Bvh;
use rayon::prelude::*;
mod aabb;
mod bvh;
mod camera;
mod helpers;
mod hittable;
mod material;
mod ray;
mod sphere;
mod world;

use camera::Camera;
use glam::Vec3A;
use hittable::{HitRecord, Hittable};
use material::Material;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::Rng;
mod moving_sphere;
use ray::Ray;
use sphere::Sphere;
use std::{thread, time};
use world::World;

use crate::moving_sphere::MovingSphere;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const SAMPLE_COUNT: usize = 30;

fn color_at(ray: &Ray, bvh: &Bvh, depth: u32) -> Vec3A {
    if let Some(rec) = bvh.hit(ray, 0.001, std::f32::MAX) {
        let mut scattered = Ray::new(Vec3A::default(), Vec3A::default(), ray.time());
        let mut attenuation = Vec3A::default();
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
                return attenuation * color_at(&scattered, bvh, depth + 1);
            } else {
                return Vec3A::new(0.0, 0.0, 0.0);
            }
        } else {
            panic!("No material wtf!");
        }
    } else {
        let t = 0.5 * (ray.dir().y + 1.0);
        (1.0 - t) * Vec3A::new(1.0, 1.0, 1.0) + t * Vec3A::new(0.5, 0.7, 1.0)
    }
}

fn generate_scene(buffer: &mut Vec<u32>) {
    let mut rng = rand::thread_rng();
    let mut world = World::default();
    world.add_object(Box::new(Sphere::new(
        Vec3A::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Vec3A::new(0.5, 0.5, 0.5),
        },
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range::<f32>(0.0, 1.0);

            let center = Vec3A::new(
                a as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2 + 0.2 * rng.gen_range::<f32>(0.0, 1.0),
                b as f32 + 0.9 * rng.gen_range(0.0, 1.0),
            );

            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let r: f32 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
                    let g: f32 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
                    let b: f32 = rng.gen_range(0.0, 1.0) * rng.gen_range(0.0, 1.0);
                    let center2 = center + 0.4 * Vec3A::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    world.add_object(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Material::Lambertian {
                            albedo: Vec3A::new(r, g, b),
                        },
                    )));
                } else if choose_mat < 0.95 {
                    let r: f32 = 0.5 * (1.0 + rng.gen_range(0.0, 1.0));
                    let g: f32 = 0.5 * (1.0 + rng.gen_range(0.0, 1.0));
                    let b: f32 = 0.5 * (1.0 + rng.gen_range(0.0, 1.0));
                    world.add_object(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            albedo: Vec3A::new(r, g, b),
                            fuzz: 0.5 * rng.gen_range(0.0, 1.0),
                        },
                    )));
                } else {
                    world.add_object(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric { ref_idx: 1.5 },
                    )));
                }
            }
        }
    }

    world.add_object(Box::new(Sphere::new(
        Vec3A::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { ref_idx: 1.5 },
    )));

    world.add_object(Box::new(Sphere::new(
        Vec3A::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3A::new(0.4, 0.2, 0.1),
        },
    )));

    world.add_object(Box::new(Sphere::new(
        Vec3A::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vec3A::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    let look_from = Vec3A::new(12.0, 1.0, 3.0);
    let look_at = Vec3A::new(1.0, 0.7, -1.0);
    let apperture = 0.0;
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3A::new(0.0, 1.0, 0.0),
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        apperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let bvh = world.generate_bvh(0.0, 1.0);
    // println!("Bvh: {:#?}", bvh);
    // panic!("WTF");

    let start = time::Instant::now();
    let chunk_size = WIDTH * 2;

    //Switch from par_iter_mut() to iter_mut() to compare with the single threaded version.
    buffer
        .par_iter_mut()
        .chunks(chunk_size)
        .enumerate()
        .for_each(|(pos, row_data)| {
            for (local_pos, data) in row_data.into_iter().enumerate() {
                let pos = pos * chunk_size + local_pos;
                let x = pos % WIDTH;
                let y = HEIGHT - pos / WIDTH;
                let mut total = Vec3A::default();
                let mut rng = rand::thread_rng();
                for _ in 0..SAMPLE_COUNT {
                    let rx = rng.gen_range(0.0, 1.0);
                    let ry = rng.gen_range(0.0, 1.0);
                    let u = (x as f32 + rx) / (WIDTH as f32);
                    let v = (y as f32 + ry) / (HEIGHT as f32);
                    let r = camera.get_ray(u, v);
                    total = total + color_at(&r, &bvh, 0);
                }
                let fcolor = total / (SAMPLE_COUNT as f32);
                let fcolor = Vec3A::new(fcolor.x.sqrt(), fcolor.y.sqrt(), fcolor.z.sqrt());
                let color_r = (fcolor.x * 255.99) as u32;
                let color_g = (fcolor.y * 255.99) as u32;
                let color_b = (fcolor.z * 255.99) as u32;
                *data = color_r << 16 | color_g << 8 | color_b;
            }
        });
    let duration = time::Instant::now() - start;
    println!("Generate took: {:?}", duration);
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; //R..G..B..R..G..B

    let mut window = Window::new(
        "Raytracing on a plane - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    generate_scene(&mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            generate_scene(&mut buffer);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        thread::sleep(time::Duration::from_millis(33));
    }
}
