use rand::{distributions::Uniform, prelude::*};
use std::cell::RefCell;
use std::ops::{Range, RangeInclusive};

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
    static DIST: Box<Uniform<f64>> = Box::new(Uniform::new(0.0, 1.0));
}

pub fn random_f64() -> f64 {
    RNG.with(|rng| DIST.with(|dist| dist.sample(&mut *rng.borrow_mut())))
}

pub fn random_f64_range(range: Range<f64>) -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen_range(range))
}

pub fn random_i32_range(range: RangeInclusive<i32>) -> i32 {
    RNG.with(|rng| rng.borrow_mut().gen_range(range))
}
