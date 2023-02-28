use std::cell::RefCell;
use rand::{prelude::*, distributions::Uniform};

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
    static DIST: Box<Uniform<f64>> = Box::new(Uniform::new(0.0, 1.0));
}

pub fn random_f64() -> f64 {
    RNG.with(|rng| DIST.with(|dist| dist.sample(&mut *rng.borrow_mut())))
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen_range(min..max))
}
