use std::rc::Rc;
use std::sync::Arc;

use crate::helpers::*;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use glam::Vec3A;
use rand;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian { texture: Arc<Box<dyn Texture>> },
    Metal { albedo: Vec3A, fuzz: f32 },
    Dielectric { ref_idx: f32 },
}

impl Default for Material {
    fn default() -> Material {
        Material::Metal {
            albedo: Vec3A::new(0.8, 0.8, 0.0),
            fuzz: 0.3,
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    hit: &HitRecord,
    attenuation: &mut Vec3A,
    scattered: &mut Ray,
) -> bool {
    match &material {
        &Material::Lambertian { texture } => {
            let target = hit.p + hit.normal + random_point_in_unit_sphere();
            *scattered = Ray::new(hit.p, target - hit.p, ray_in.time());
            *attenuation = texture.color(hit.u, hit.v, hit.p);
            true
        }
        &Material::Metal { albedo, fuzz } => {
            let reflected = reflect(ray_in.dir(), hit.normal);
            *scattered = Ray::new(
                hit.p,
                reflected + *fuzz * random_point_in_unit_sphere(),
                ray_in.time(),
            );
            *attenuation = *albedo;

            Vec3A::dot(scattered.dir(), hit.normal) > 0.0
        }
        &Material::Dielectric { ref_idx } => {
            let outward_normal;
            let reflected = reflect(ray_in.dir(), hit.normal);
            let ni_over_nt: f32;
            let cosine;
            let reflect_prob;

            *attenuation = Vec3A::new(1.0, 1.0, 1.0);

            if Vec3A::dot(ray_in.dir(), hit.normal) > 0.0 {
                outward_normal = -hit.normal;
                ni_over_nt = *ref_idx;
                cosine = ref_idx * Vec3A::dot(ray_in.dir(), hit.normal) / ray_in.dir().length();
            } else {
                outward_normal = hit.normal;
                ni_over_nt = 1.0 / *ref_idx;
                cosine = -Vec3A::dot(ray_in.dir(), hit.normal) / ray_in.dir().length();
            }
            let refracted = refract(ray_in.dir(), outward_normal, ni_over_nt);
            if refracted.is_some() {
                reflect_prob = schlick(cosine, *ref_idx);
            } else {
                reflect_prob = 1.0;
            }

            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0.0, 1.0);

            if random_number < reflect_prob {
                *scattered = Ray::new(hit.p, reflected, ray_in.time());
            } else {
                *scattered = Ray::new(hit.p, refracted.unwrap(), ray_in.time());
            }

            return true;
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn random_point_in_unit_sphere() -> Vec3A {
    let mut rng = rand::thread_rng();

    loop {
        let rand_x = rng.gen_range(-1.0, 1.0);
        let rand_y = rng.gen_range(-1.0, 1.0);
        let rand_z = rng.gen_range(-1.0, 1.0);
        let v = Vec3A::new(rand_x, rand_y, rand_z);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}
