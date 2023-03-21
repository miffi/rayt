use crate::aabb::Aabb;
use crate::hittable_list::HittableList;
use crate::{hittable as h, util};
use std::ops::Range;
use std::rc::Rc;

pub struct BvhNode {
    left: Rc<dyn h::Hittable>,
    right: Rc<dyn h::Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &HittableList, time_range: Range<f64>) -> Self {
        Self::make(&mut list.objects.clone()[..], time_range)
    }

    fn box_compare(
        a: &Rc<dyn h::Hittable>,
        b: &Rc<dyn h::Hittable>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let box_a = a.bounding_box(0.0..0.0);
        let box_b = b.bounding_box(0.0..0.0);
        if box_a.is_none() || box_b.is_none() {
            eprintln!("No bouding box in bvh_node constructor.\n");
        }

        if box_a.unwrap().min()[axis] < box_b.unwrap().min()[axis] {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }

    fn make(objects: &mut [Rc<dyn h::Hittable>], time_range: Range<f64>) -> Self {
        let axis = util::random_i32_range(0..=2) as usize;

        let (left, right) = if objects.len() == 1 {
            (objects[0].clone(), objects[0].clone())
        } else if objects.len() == 2 {
            if Self::box_compare(&objects[0], &objects[1], axis) == std::cmp::Ordering::Less {
                (objects[0].clone(), objects[1].clone())
            } else {
                (objects[1].clone(), objects[0].clone())
            }
        } else {
            objects.sort_by(|a, b| Self::box_compare(a, b, axis));

            let mid = objects.len() / 2;
            let left: Rc<dyn h::Hittable> =
                Rc::new(Self::make(&mut objects[..mid], time_range.clone()));
            let right: Rc<dyn h::Hittable> =
                Rc::new(Self::make(&mut objects[mid..], time_range.clone()));
            (left, right)
        };

        Self {
            bbox: Aabb::surrounding_box(
                left.bounding_box(time_range.clone()).unwrap(),
                right.bounding_box(time_range.clone()).unwrap(),
            ),
            left,
            right,
        }
    }
}

impl h::Hittable for BvhNode {
    fn hit(&self, r: &crate::ray::Ray, range: Range<f64>) -> Option<h::HitRecord> {
        if !self.bbox.hit(r, range.clone()) {
            None
        } else {
            if let Some(hit_left) = self.left.hit(r, range.clone()) {
                self.right
                    .hit(r, range.start..hit_left.t)
                    .or(Some(hit_left))
            } else {
                self.right.hit(r, range)
            }
        }
    }

    fn bounding_box(&self, _time_range: Range<f64>) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
