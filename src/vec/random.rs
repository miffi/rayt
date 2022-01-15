use rand::Rng;

pub fn in_unit_sphere(rng: &mut rand::rngs::ThreadRng) -> super::Vec3 {
    loop {
        let x = super::Vec3::from_fn(|_, _| rng.gen_range(-1.0..1.0));
        if x.norm_squared() < 1.0 {
            return x;
        }
    }
}

pub fn unit_vector(rng: &mut rand::rngs::ThreadRng) -> super::Vec3 {
    in_unit_sphere(rng).normalize()
}

#[allow(dead_code)]
pub fn in_hemisphere(normal: &super::Vec3, rng: &mut rand::rngs::ThreadRng) -> super::Vec3 {
    let in_unit_sphere = in_unit_sphere(rng);
    if in_unit_sphere.dot(&normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
