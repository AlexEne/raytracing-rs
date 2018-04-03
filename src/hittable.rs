use ray::Ray;
use vec3::Vec3;
use material::MaterialHelper;
use std::boxed::Box;

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Option<MaterialHelper>
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool;
}
