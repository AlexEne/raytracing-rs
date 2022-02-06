use std::cmp::Ordering;

use rand::{thread_rng, Rng};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
};

pub enum BvhContents {
    Node { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

pub struct Bvh {
    bounding_box: AABB,
    contents: BvhContents,
    size: usize,
}

impl Bvh {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, t0: f32, t1: f32) -> Bvh {
        fn axis_range(axis: usize, objs: &[Box<dyn Hittable>], t0: f32, t1: f32) -> f32 {
            let range = objs
                .iter()
                .fold((std::f32::MAX, std::f32::MIN), |range, o| {
                    let bb = o.bounding_box(t0, t1);
                    let min = bb.min[axis].min(bb.max[axis]);
                    let max = bb.min[axis].max(bb.max[axis]);

                    (range.0.min(min), range.1.max(max))
                });

            range.1 - range.0
        }

        // let axis = rng.gen_range(0, 3);
        let axis = {
            let mut ranges = [
                (0, axis_range(0, &objects, t0, t1)),
                (1, axis_range(1, &objects, t0, t1)),
                (2, axis_range(2, &objects, t0, t1)),
            ];

            ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            ranges[0].0
        };

        objects.sort_unstable_by(|a, b| {
            let abb = a.bounding_box(t0, t1);
            let bbb = b.bounding_box(t0, t1);

            let av = abb.min[axis] + abb.max[axis];
            let bv = bbb.min[axis] + bbb.max[axis];
            av.partial_cmp(&bv).unwrap()
        });

        match objects.len() {
            0 => panic!("Must have at least 1 object to insert"),
            1 => Bvh {
                bounding_box: objects[0].bounding_box(t0, t1),
                contents: BvhContents::Leaf(objects.pop().unwrap()),
                size: 1,
            },
            _ => {
                let right = Box::new(Bvh::new(
                    objects.drain(objects.len() / 2..).collect(),
                    t0,
                    t1,
                ));

                let left = Box::new(Bvh::new(objects, t0, t1));

                Bvh {
                    bounding_box: left.bounding_box.union(&right.bounding_box),
                    size: left.size + right.size,
                    contents: BvhContents::Node { left, right },
                }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &crate::ray::Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, tmin, tmax) {
            match &self.contents {
                BvhContents::Node { left, right } => {
                    let hit_left = left.hit(ray, tmin, tmax);
                    let hit_right = right.hit(ray, tmin, tmax);

                    match (hit_left, hit_right) {
                        (h, None) | (None, h) => h,
                        (Some(hl), Some(hr)) => {
                            if hl.t < hr.t {
                                Some(hl)
                            } else {
                                Some(hr)
                            }
                        }
                    }
                }
                BvhContents::Leaf(obj) => obj.hit(ray, tmin, tmax),
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f32, _: f32) -> AABB {
        self.bounding_box.clone()
    }
}
