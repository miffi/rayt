use crate::{hittable, ray, vec};

pub struct Sphere {
    center: vec::Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: vec::Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().norm_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

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
