use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec;
use std::ops::Range;
use std::rc::Rc;

pub struct HitRecord {
    pub p: vec::Point,
    pub normal: vec::Vec,
    pub t: f64,
    pub uv: (f64, f64),
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        t: f64,
        r: &Ray,
        normal: vec::Vec,
        material: Rc<dyn Material>,
        uv: (f64, f64),
    ) -> Self {
        let (normal, front_face) = if r.direction().dot(&normal) < 0.0 {
            (normal, true)
        } else {
            (-normal, false)
        };

        Self {
            p: r.at(t),
            normal,
            t,
            uv,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, range: Range<f64>) -> Option<HitRecord>;
    fn bounding_box(&self, time_range: Range<f64>) -> Option<Aabb>;
}
