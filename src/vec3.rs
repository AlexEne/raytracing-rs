use std::f32;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }
    pub fn y(&self) -> f32 {
        self.data[1]
    }
    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn r(&self) -> f32 {
        self.data[0]
    }
    pub fn g(&self) -> f32 {
        self.data[1]
    }
    pub fn b(&self) -> f32 {
        self.data[2]
    }


    pub fn normalize(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3::new(
            self.data[0] * k,
            self.data[1] * k,
            self.data[2] * k,
        )
    }

    pub fn length(&self) -> f32 {
        self.square_length().sqrt()
    }

    pub fn square_length(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

pub fn dot(first: &Vec3, other: &Vec3) -> f32 {
    first.x() * other.x() + first.y() * other.y() + first.z() * other.z()
}

pub fn reflect(v: &Vec3, &n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, &n) * n
}

pub fn refract(&v: &Vec3, &n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, k: f32) -> Vec3 {
        Vec3 {
            data: [self.x() * k, self.y() * k, self.z() * k],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [other.x() * self, other.y() * self, other.z() * self],
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.x() / other.x(),
                self.y() / other.y(),
                self.z() / other.z(),
            ],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, k: f32) -> Vec3 {
        Vec3 {
            data: [self.x() / k, self.y() / k, self.z() / k],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        data: [
            a.y() * b.z() - a.z() * b.y(),
            -(a.x() * b.z() - a.z() * b.x()),
            a.x() * b.y() - a.y() * b.x(),
        ],
    }
}