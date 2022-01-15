use nalgebra as na;
use rand::{self, Rng, SeedableRng};

pub type Vec3 = na::Vector3<f64>;

pub fn random_in_unit_sphere(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    loop {
        let x = Vec3::from_fn(|_, _| rng.gen_range(-1.0..1.0));
        if x.norm_squared() < 1.0 {
            return x;
        }
    }
}

pub fn random_unit_vector(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    random_in_unit_sphere(rng)
}

pub fn random_in_hemisphere(normal: &Vec3, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere.dot(&normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
