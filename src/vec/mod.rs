use nalgebra as na;

pub mod random;

pub type Vec3 = na::Vector3<f64>;

pub fn near_zero(vec: Vec3) -> bool {
    const S: f64 = 1e-8;
    vec.iter().all(|x| x.abs() < S)
}

pub fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    return vec - 2.0 * vec.dot(normal) * normal;
}

pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(normal).min(1.0);
    let out_perp = etai_over_etat * (uv + cos_theta * normal);
    let out_parallel = -((1.0 - out_perp.norm_squared()).abs().sqrt()) * normal;
    out_perp + out_parallel
}
