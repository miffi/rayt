use crate::{ray, vec};

use nalgebra as na;

pub struct Camera {
    origin: vec::Vec3,
    lower_left_corner: vec::Vec3,

    horizontal: vec::Vec3,
    vertical: vec::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = na::vector![0.0, 0.0, 0.0];
        let horizontal = na::vector![viewport_width, 0.0, 0.0];
        let vertical = na::vector![0.0, viewport_height, 0.0];

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - na::vector!(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn origin(&self) -> &vec::Vec3 {
        &self.origin
    }

    pub fn lower_left_corner(&self) -> &vec::Vec3 {
        &self.lower_left_corner
    }

    pub fn horizontal(&self) -> &vec::Vec3 {
        &self.horizontal
    }

    pub fn vertical(&self) -> &vec::Vec3 {
        &self.vertical
    }
}
