use rand::{self, Rng};

pub fn random_in_unit_sphere() -> super::Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let x = super::Vec3::from_fn(|_, _| rng.gen_range(-1.0..1.0));
        if x.norm_squared() < 1.0 {
            return x;
        }
    }
}

pub fn unit_vector() -> super::Vec3 {
    random_in_unit_sphere().normalize()
}

#[allow(dead_code)]
pub fn in_hemisphere(normal: &super::Vec3) -> super::Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn in_unit_disk() -> super::Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = super::Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}
