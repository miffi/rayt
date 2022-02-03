use crate::{ray, vec};

pub struct Camera {
    origin: vec::Vec3,
    lower_left_corner: vec::Vec3,

    horizontal: vec::Vec3,
    vertical: vec::Vec3,

    u: vec::Vec3,
    v: vec::Vec3,
    w: vec::Vec3,

    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: vec::Vec3,
        lookat: vec::Vec3,
        vup: vec::Vec3,
        vfov: f64, // The vertical field-of-view in degrees
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
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,

            u,
            v,
            w,

            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        let rd = self.lens_radius * vec::random::in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        ray::Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
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
