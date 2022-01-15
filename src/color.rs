use std::io::{Stdout, Write};

use crate::vec;
use itertools::Itertools;

pub fn write_color(
    file: &mut Stdout,
    pixel_color: vec::Vec3,
    samples_per_pixel: u32,
) -> Result<(), std::io::Error> {
    let scale = 1.0 / samples_per_pixel as f64;

    write!(
        file,
        "{}\n",
        pixel_color
            .map(|x| (255.999 * (x * scale).sqrt().clamp(0.0, 0.999)) as u32)
            .iter()
            .format(" "),
    )
}
