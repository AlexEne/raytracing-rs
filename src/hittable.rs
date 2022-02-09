use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

pub struct HitRecord {
    pub p: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub material: Option<Material>,
}

pub trait Hittable: Send + Sync + std::fmt::Debug {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB;
}
