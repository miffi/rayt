use crate::material::Material;
use crate::ray::Ray;
use crate::vec;
use std::rc::Rc;

pub struct HitRecord {
    pub p: vec::Point,
    pub normal: vec::Vec,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f64, r: &Ray, normal: vec::Vec, material: Rc<dyn Material>) -> Self {
        let (normal, front_face) = if r.direction().dot(&normal) < 0.0 {
            (normal, true)
        } else {
            (-normal, false)
        };

        Self {
            p: r.at(t),
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, range: (f64, f64)) -> Option<HitRecord>;
}
