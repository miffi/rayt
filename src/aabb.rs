use crate::ray::Ray;
use crate::vec::{Point, Vec};
use std::ops::Range;

#[derive(Clone)]
pub struct Aabb {
    maximum: Point,
    minimum: Point,
}

impl Aabb {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        Self { minimum, maximum }
    }

    pub fn min(&self) -> &Point {
        &self.minimum
    }

    pub fn max(&self) -> &Point {
        &self.maximum
    }

    pub fn hit(&self, r: &Ray, time_range: Range<f64>) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];

            let t0 = (self.minimum[i] - r.origin()[i]) * inv_d;
            let t1 = (self.maximum[i] - r.origin()[i]) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            let t_min = if t0 > time_range.start {
                t0
            } else {
                time_range.start
            };
            let t_max = if t1 < time_range.end {
                t1
            } else {
                time_range.end
            };

            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        Aabb::new(
            Vec::new(
                box0.minimum.x.min(box1.minimum.x),
                box0.minimum.y.min(box1.minimum.y),
                box0.minimum.z.min(box1.minimum.z),
            ),
            Vec::new(
                box0.maximum.x.max(box1.maximum.x),
                box0.maximum.y.max(box1.maximum.y),
                box0.maximum.z.max(box1.maximum.z),
            ),
        )
    }
}
