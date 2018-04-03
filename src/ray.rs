use vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    dir: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        let mut dir = dir;
        dir.normalize();
        Ray {
            origin: origin,
            dir: dir
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}