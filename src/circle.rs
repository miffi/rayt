use crate::aabb::Aabb;
use crate::hittable as h;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Point, Vec};
use std::f64::consts::PI;
use std::ops::Range;
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

    fn get_uv(p: &Point) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl h::Hittable for Sphere {
    fn hit(&self, r: &Ray, range: Range<f64>) -> Option<h::HitRecord> {
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
        if !range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !range.contains(&root) {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;

        Some(h::HitRecord::new(
            root,
            r,
            outward_normal,
            self.material.clone(),
            Self::get_uv(&outward_normal)
        ))
    }

    fn bounding_box(&self, _time_range: Range<f64>) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec::from_element(self.radius),
            self.center + Vec::from_element(self.radius),
        ))
    }
}
