use crate::ray;

pub struct HitRecord {
    p: ray::Vec3,
    normal: ray::Vec3,
    t: f64,
    front_face: bool,
}

pub fn correct_normal_direction(r: &ray::Ray, outward_normal: ray::Vec3) -> (bool, ray::Vec3) {
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
    pub fn new(p: ray::Vec3, (front_face, normal): (bool, ray::Vec3), t: f64) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> &ray::Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> ray::Vec3 {
        self.p
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
