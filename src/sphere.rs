use crate::{hittable, ray};

use nalgebra_glm as glm;

pub struct Sphere {
    center: ray::Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new (center: ray::Vec3, radius: f64) -> Self {
        Sphere {center, radius}
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let oc = r.origin() - self.center;
        let a = glm::length2(r.direction());
        let half_b = glm::dot(&oc, r.direction());
        let c = glm::length2(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        Some(hittable::HitRecord::new(
            r.at(root), // point of intersection
            hittable::correct_normal_direction(r, (r.at(root) - self.center) / self.radius), // normal made to face outward
            root,
        ))
    }
}