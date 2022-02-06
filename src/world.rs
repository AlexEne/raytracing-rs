use glam::Vec3A;

use crate::aabb::AABB;
use crate::bvh::Bvh;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::boxed::Box;

#[derive(Default, Debug)]
pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn add_object(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn generate_bvh(self, t0: f32, t1: f32) -> Bvh {
        Bvh::new(self.objects, t0, t1)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut closest = tmax;
        let mut hit = None;

        for obj in self.objects.iter() {
            if let Some(hit_record) = obj.hit(ray, tmin, tmax) {
                if hit_record.t < closest {
                    closest = hit_record.t;
                    hit = Some(hit_record)
                }
            }
        }

        hit
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> AABB {
        AABB::new(Vec3A::ONE, Vec3A::ONE)
    }
}
