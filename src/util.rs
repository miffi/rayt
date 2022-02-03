use rand::{self, Rng};

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
