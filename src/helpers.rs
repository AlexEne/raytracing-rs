use glam::Vec3A;

pub fn reflect(v: Vec3A, n: Vec3A) -> Vec3A {
    v - 2.0 * Vec3A::dot(v, n) * n
}

pub fn refract(v: Vec3A, n: Vec3A, ni_over_nt: f32) -> Option<Vec3A> {
    let uv = v.normalize();
    let dt = Vec3A::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}
