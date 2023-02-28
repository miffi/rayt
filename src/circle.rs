use crate::hittable as h;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Point;
use std::rc::Rc;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl h::Hittable for Sphere {
    fn hit(&self, r: &Ray, (t_min, t_max): (f64, f64)) -> Option<h::HitRecord> {
        let oc = r.origin() - self.center;

        let a = r.direction().dot(r.direction());
        let half_b = oc.dot(r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
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

        Some(h::HitRecord::new(
            root,
            r,
            (r.at(root) - self.center) / self.radius,
            self.material.clone()
        ))
    }
}
