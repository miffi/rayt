use crate::vec::{Vec, Point};

pub struct Ray {
    orig: Point,
    dir: Vec,
}

impl Ray {
    pub fn new(orig: Vec, dir: Vec) -> Ray {
        Ray {
            orig,
            dir,
        }
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
}
