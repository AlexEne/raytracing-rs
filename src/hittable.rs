use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

pub struct HitRecord {
    pub p: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub material: Option<Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Vec3A::ZERO,
            normal: Vec3A::ONE,
            t: 0.0,
            material: None,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool;
}
