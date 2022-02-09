use glam::Vec3A;

pub trait Texture: Send + Sync + core::fmt::Debug {
    fn color(&self, u: f32, v: f32, p: Vec3A) -> Vec3A;
}

#[derive(Debug)]
pub struct SolidColor {
    color: Vec3A,
}

impl SolidColor {
    pub fn new(color: Vec3A) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn color(&self, _: f32, _: f32, _: Vec3A) -> Vec3A {
        self.color
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    odd: Vec3A,
    even: Vec3A,
}

impl CheckerTexture {
    pub fn new(odd: Vec3A, even: Vec3A) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        let sines = f32::sin(20.0 * p.x) * f32::sin(10.0 * p.y) * f32::sin(10.0 * p.z);

        if sines < 0.0 {
            self.odd
        } else {
            self.even
        }
    }
}
