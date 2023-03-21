use crate::vec::{Point, Vec};

pub struct Ray {
    orig: Point,
    dir: Vec,
    time: f64,
}

impl Ray {
    pub fn new(orig: Vec, dir: Vec, time: f64) -> Ray {
        Ray { orig, dir, time }
    }

    pub fn at(&self, t: f64) -> Vec {
        self.orig + self.dir * t
    }

    pub fn unit_direction(&self) -> Vec {
        self.dir.normalize()
    }

    pub fn origin(&self) -> &Point {
        &self.orig
    }

    pub fn direction(&self) -> &Vec {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
