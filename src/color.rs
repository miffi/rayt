use std::io::{Stdout, Write};

use crate::ray;

pub fn write_color(file: &mut Stdout, pixel_color: ray::Vec3) -> Result<(), std::io::Error> {
    write!(
        file,
        "{} {} {}\n",
        (255.999 * pixel_color[0]) as u32,
        (255.999 * pixel_color[1]) as u32,
        (255.999 * pixel_color[2]) as u32,
    )
}
