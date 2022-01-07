use crate::ray;

use nalgebra_glm as glm;

pub struct Camera {
    origin: ray::Vec3,
    lower_left_corner: ray::Vec3,

    horizontal: ray::Vec3,
    vertical: ray::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = glm::vec3(0.0, 0.0, 0.0);
        let horizontal = glm::vec3(viewport_width, 0.0, 0.0);
        let vertical = glm::vec3(0.0, viewport_height, 0.0);

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - glm::vec3(0.0, 0.0, focal_length);

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

    pub fn origin(&self) -> &ray::Vec3 {
        &self.origin
    }

    pub fn lower_left_corner(&self) -> &ray::Vec3 {
        &self.lower_left_corner
    }

    pub fn horizontal(&self) -> &ray::Vec3 {
        &self.horizontal
    }

    pub fn vertical(&self) -> &ray::Vec3 {
        &self.vertical
    }
}
