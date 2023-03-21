use std::fs::File;
use std::rc::Rc;

use crate::vec::{Color, Point};

pub trait Texture {
    fn value(&self, uv: (f64, f64), p: &Point) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Color::new(r, g, b),
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, (_u, _v): (f64, f64), _p: &Point) -> Color {
        self.color
    }
}

pub struct Checker {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl Texture for Checker {
    fn value(&self, uv: (f64, f64), p: &Point) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        let texture = if sines < 0.0 { &self.odd } else { &self.even };
        texture.value(uv, p)
    }
}

impl Checker {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

pub struct Image {
    data: Vec<u8>,
    info: png::OutputInfo,
    bytes_per_pixel: usize,
}

impl Image {
    pub fn from_png_file(file: &File) -> Result<Self, png::DecodingError> {
        let mut reader = png::Decoder::new(file).read_info()?;

        let mut data = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut data)?;
        eprintln!("{:?}", info);

        Ok(Self {
            data,
            bytes_per_pixel: Self::calc_pixel_bytes(&info),
            info,
        })
    }

    fn calc_pixel_bytes(info: &png::OutputInfo) -> usize {
        use png::BitDepth::*;

        let bit_depth = match info.bit_depth {
            One => 1,
            Two => 2,
            Four => 4,
            Eight => 8,
            Sixteen => 16,
        };
        return bit_depth / 8 * 3;
    }
}

impl Texture for Image {
    fn value(&self, (u, v): (f64, f64), _p: &Point) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = ((u * self.info.width as f64) as usize).clamp(0, self.info.width as usize - 1);
        let mut j = ((v * self.info.height as f64) as usize).clamp(0, self.info.height as usize - 1);

        if i >= self.info.width as usize {
            i = self.info.width as usize - 1;
        }

        if j >= self.info.height as usize {
            j = self.info.height as usize - 1;
        }

        let index = j * self.info.line_size + i * self.bytes_per_pixel;
        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * self.data[index] as f64,
            color_scale * self.data[index + 1] as f64,
            color_scale * self.data[index + 2] as f64,
        )
    }
}
