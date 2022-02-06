use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

#[derive(Debug)]
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let a = Vec3A::dot(ray.dir(), ray.dir());
        let b = Vec3A::dot(oc, ray.dir());
        let c = Vec3A::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: Some(self.material),
                });
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);

                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: Some(self.material),
                });
            }
        }

        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> AABB {
        let r = Vec3A::new(self.radius, self.radius, self.radius);
        AABB::new(self.center - r, self.center + r)
    }
}
