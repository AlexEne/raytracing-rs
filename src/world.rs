use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::boxed::Box;

#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn add_object(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let mut hit_something = false;
        let mut closest = tmax;
        for obj in self.objects.iter() {
            let mut tmp_rec = HitRecord::default();

            if obj.hit(ray, tmin, tmax, &mut tmp_rec) {
                if tmp_rec.t < closest {
                    hit_something = true;
                    closest = tmp_rec.t;
                    rec.t = tmp_rec.t;
                    rec.p = tmp_rec.p;
                    rec.normal = tmp_rec.normal;
                    rec.material = tmp_rec.material;
                }
            }
        }

        hit_something
    }
}
