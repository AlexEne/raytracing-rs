use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use glam::Vec3A;

#[derive(Debug)]
pub struct MovingSphere {
    center0: Vec3A,
    center1: Vec3A,
    t0: f32,
    t1: f32,
    radius: f32,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3A,
        center1: Vec3A,
        t0: f32,
        t1: f32,
        radius: f32,
        material: Material,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            t0,
            t1,
            radius,
            material,
        }
    }
}

impl MovingSphere {
    fn center(&self, t: f32) -> Vec3A {
        return self.center0
            + ((t - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0);
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());

        let a = Vec3A::dot(ray.dir(), ray.dir());
        let b = Vec3A::dot(oc, ray.dir());
        let c = Vec3A::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let (u, v) = Sphere::get_uv(&p);
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center(ray.time())) / self.radius,
                    u,
                    v,
                    material: Some(self.material.clone()),
                });
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let (u, v) = Sphere::get_uv(&p);
                return Some(HitRecord {
                    t: temp,
                    p,
                    u,
                    v,
                    normal: (p - self.center(ray.time())) / self.radius,
                    material: Some(self.material.clone()),
                });
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> AABB {
        let r = Vec3A::new(self.radius, self.radius, self.radius);
        let start_aabb = AABB::new(self.center(t0) - r, self.center(t0) + r);
        let end_aabb = AABB::new(self.center(t1) - r, self.center(t1) + r);

        start_aabb.union(&end_aabb)
    }
}
