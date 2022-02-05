use glam::Vec3A;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec3A,
    dir: Vec3A,
}

impl Ray {
    pub fn new(origin: Vec3A, dir: Vec3A) -> Ray {
        Ray {
            origin: origin,
            dir: dir.normalize(),
        }
    }

    pub fn origin(&self) -> Vec3A {
        self.origin
    }

    pub fn dir(&self) -> Vec3A {
        self.dir
    }

    pub fn point_at(&self, t: f32) -> Vec3A {
        self.origin + self.dir * t
    }
}
