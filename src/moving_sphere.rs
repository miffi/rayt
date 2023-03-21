use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Point, Vec};
use std::f64::consts::PI;
use std::ops::Range;
use std::rc::Rc;

pub struct MovingSphere {
    center0: Point,
    center1: Point,
    time_range: Range<f64>,
    radius: f64,
    material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point,
        center1: Point,
        time_range: Range<f64>,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time_range,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point {
        self.center0
            + (time - self.time_range.start) / (self.time_range.end - self.time_range.start)
                * (self.center1 - self.center0)
    }

    fn get_uv(p: &Point) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, range: Range<f64>) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());

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

        let outward_normal = (r.at(root) - self.center(r.time())) / self.radius;

        Some(HitRecord::new(
            root,
            r,
            outward_normal,
            self.material.clone(),
            Self::get_uv(&outward_normal),
        ))
    }

    fn bounding_box(&self, time_range: Range<f64>) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time_range.start) - Vec::from_element(self.radius),
            self.center(time_range.start) + Vec::from_element(self.radius),
        );

        let box1 = Aabb::new(
            self.center(time_range.end) - Vec::from_element(self.radius),
            self.center(time_range.end) + Vec::from_element(self.radius),
        );

        Some(Aabb::surrounding_box(box0, box1))
    }
}
