use crate::vec;

pub struct Ray {
    origin: vec::Vec3,
    direction: vec::Vec3,
}

impl Ray {
    pub fn new(origin: vec::Vec3, direction: vec::Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &vec::Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &vec::Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> vec::Vec3 {
        self.origin + t * self.direction
    }
}
