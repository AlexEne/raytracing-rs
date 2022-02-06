use crate::ray::Ray;
use glam::Vec3A;

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl AABB {
    pub fn new(min: Vec3A, max: Vec3A) -> AABB {
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for axis in 0..3 {
            let r_origin = ray.origin()[axis];
            let inv_r_dir = 1.0 / ray.dir()[axis];

            let t0 = f32::min(
                (self.min[axis] - r_origin) * inv_r_dir,
                (self.max[axis] - r_origin) * inv_r_dir,
            );

            let t1 = f32::max(
                (self.min[axis] - r_origin) * inv_r_dir,
                (self.max[axis] - r_origin) * inv_r_dir,
            );

            let t_min = f32::max(t0, t_min);
            let t_max = f32::min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn union(&self, other: &AABB) -> AABB {
        AABB {
            min: Vec3A::new(
                f32::min(self.min.x, other.min.x),
                f32::min(self.min.y, other.min.y),
                f32::min(self.min.z, other.min.z),
            ),
            max: Vec3A::new(
                f32::max(self.max.x, other.max.x),
                f32::max(self.max.y, other.max.y),
                f32::max(self.max.z, other.max.z),
            ),
        }
    }
}
