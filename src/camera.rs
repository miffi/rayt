use crate::ray::Ray;
use crate::util;
use crate::vec::{self, Point, Vec};
use std::ops::Range;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec,
    vertical: Vec,
    w: Point,
    u: Point,
    v: Point,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64, time_range: Range<f64>) -> Ray {
        let rd = self.lens_radius * vec::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            util::random_f64_range(time_range),
        )
    }
}
