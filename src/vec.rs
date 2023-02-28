use crate::util;
use nalgebra as na;

pub type Vec = na::Vector3<f64>;
pub type Point = na::Vector3<f64>;
pub type Color = na::Vector3<f64>;

pub fn write_color(
    writer: &mut dyn std::io::Write,
    pixel_color: Color,
    samples_per_pixel: u32,
) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;

    let f = |x: f64| (256.0 * (x * scale).sqrt().clamp(0.0, 0.999)) as u32;

    write!(
        writer,
        "{} {} {}\n",
        f(pixel_color.x),
        f(pixel_color.y),
        f(pixel_color.z)
    )
}

pub fn random() -> Vec {
    Vec::new(util::random_f64(), util::random_f64(), util::random_f64())
}

pub fn random_range(min: f64, max: f64) -> Vec {
    Vec::new(
        util::random_f64_range(min, max),
        util::random_f64_range(min, max),
        util::random_f64_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec {
    loop {
        let p = random_range(-1.0, 1.0);
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec {
    random_in_unit_sphere().normalize()
}

pub fn random_in_unit_disk() -> Vec {
    loop {
        let p = Vec::new(
            util::random_f64_range(-1.0, 1.0),
            util::random_f64_range(-1.0, 1.0),
            0.0,
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub fn near_zero(vec: Vec) -> bool {
    let s = 1e-8;
    vec.iter().all(|x| x.abs() < s)
}

pub fn reflect(vec: &Vec, normal: &Vec) -> Vec {
    vec - 2.0 * vec.dot(&normal) * normal
}

pub fn refract(uv: &Vec, normal: &Vec, etai_over_etat: f64) -> Vec {
    let cos_theta = normal.dot(&-uv).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -(((1.0 - r_out_perp.norm_squared()) as f64).abs().sqrt()) * normal;

    r_out_perp + r_out_parallel
}
