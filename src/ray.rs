use nalgebra_glm as glm;

pub type Vec3 = glm::DVec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(self: &Self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(self: &Self) -> &Vec3 {
        &self.direction
    }

    pub fn at(self: &Self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
