use hittable::HitRecord;
use rand;
use rand::Rng;
use ray::Ray;
use vec3;
use vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ref_idx: f32 },
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    hit: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = hit.p + hit.normal + random_point_in_unit_sphere();
            *scattered = Ray::new(hit.p, target - hit.p);
            *attenuation = albedo;
            true
        }
        &Material::Metal {
            albedo,
            fuzz,
        } => {
            let reflected = vec3::reflect(&ray_in.dir(), &hit.normal);
            *scattered = Ray::new(hit.p, reflected + fuzz * random_point_in_unit_sphere());
            *attenuation = albedo;

            vec3::dot(&scattered.dir(), &hit.normal) > 0.0
        }
        &Material::Dielectric { ref_idx } => {
            let mut outward_normal = Vec3::new(0.0, 0.0, 0.0);
            let reflected = vec3::reflect(&ray_in.dir(), &hit.normal);
            let mut ni_over_nt = 0.0;
            let mut cosine = 0.0;
            let mut reflect_prob = 1.0;
            
            *attenuation = Vec3::new(1.0, 1.0, 1.0);

            if vec3::dot(&ray_in.dir(), &hit.normal) > 0.0 {
                outward_normal = -hit.normal;
                ni_over_nt = ref_idx;
                cosine = ref_idx * vec3::dot(&ray_in.dir(), &hit.normal)/ray_in.dir().length();
            } else {
                outward_normal = hit.normal;
                ni_over_nt = 1.0 / ref_idx;
                cosine = -vec3::dot(&ray_in.dir(), &hit.normal) / ray_in.dir().length();
            }
            let refracted = vec3::refract(&ray_in.dir(), &outward_normal, ni_over_nt);
            if refracted.is_some() {
                reflect_prob = schlick(cosine, ref_idx);
            } else {
                reflect_prob = 1.0;
            }

            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0.0, 1.0);

            if random_number < reflect_prob {
                *scattered = Ray::new(hit.p, reflected);
            } else {
                *scattered = Ray::new(hit.p, refracted.unwrap());
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

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let rand_x = rng.gen_range(-1.0, 1.0);
        let rand_y = rng.gen_range(-1.0, 1.0);
        let rand_z = rng.gen_range(-1.0, 1.0);
        let v = Vec3::new(rand_x, rand_y, rand_z);
        if v.square_length() < 1.0 {
            return v;
        }
    }
}
