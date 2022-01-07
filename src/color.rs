use std::io::{Stdout, Write};

use crate::ray;

pub fn write_color(file: &mut Stdout, pixel_color: ray::Vec3, samples_per_pixel: u32) -> Result<(), std::io::Error> {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color[0] * scale;
    let g = pixel_color[1] * scale;
    let b = pixel_color[2] * scale;

    write!(
        file,
        "{} {} {}\n",
        (255.999 * r.clamp(0.0, 0.999)) as u32,
        (255.999 * g.clamp(0.0, 0.999)) as u32,
        (255.999 * b.clamp(0.0, 0.999)) as u32,
    )
}
