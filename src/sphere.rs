use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

pub struct Sphere {
    center: Vec3A,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;

        let a = Vec3A::dot(ray.dir(), ray.dir());
        let b = Vec3A::dot(oc, ray.dir());
        let c = Vec3A::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material);

                return true;
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(self.material);

                return true;
            }
        }

        false
    }
}
