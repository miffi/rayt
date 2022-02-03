use std::rc::Rc;

use crate::{material, ray, vec};

pub struct HitRecord {
    p: vec::Vec3,
    normal: vec::Vec3,
    t: f64,
    material: Rc<dyn material::Material>,
    front_face: bool,
}

pub fn correct_normal_direction(r: &ray::Ray, outward_normal: vec::Vec3) -> (bool, vec::Vec3) {
    let front_face = r.direction().dot(&outward_normal) < 0.0;
    (
        front_face,
        if front_face {
            outward_normal
        } else {
            -outward_normal
        },
    )
}

impl HitRecord {
    pub fn new(
        p: vec::Vec3,
        (front_face, normal): (bool, vec::Vec3),
        material: Rc<dyn material::Material>,
        t: f64,
    ) -> Self {
        HitRecord {
            p,
            normal,
            t,
            material,
            front_face,
        }
    }

    pub fn normal(&self) -> &vec::Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> vec::Vec3 {
        self.p
    }

    pub fn material(&self) -> &dyn material::Material {
        self.material.as_ref()
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
