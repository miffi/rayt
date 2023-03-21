use crate::aabb::Aabb;
use crate::hittable as h;
use crate::ray::Ray;
use std::ops::Range;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn h::Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Rc<dyn h::Hittable>) {
        self.objects.push(obj);
    }
}

impl h::Hittable for HittableList {
    fn hit(&self, r: &Ray, range: Range<f64>) -> Option<h::HitRecord> {
        let f = |(closest, rec), obj: &Rc<dyn h::Hittable>| {
            if let Some(temp_rec) = obj.hit(r, range.start..closest) {
                (temp_rec.t, Some(temp_rec))
            } else {
                (closest, rec)
            }
        };

        self.objects.iter().fold((range.end, None), f).1
    }

    fn bounding_box(&self, time_range: Range<f64>) -> Option<Aabb> {
        if self.objects.is_empty() {
            None
        } else {
            let mut bbox = None;
            let mut first = true;

            for object in self.objects.iter() {
                if let Some(ab) = object.bounding_box(time_range.clone()) {
                    bbox = if first {
                        Some(ab)
                    } else {
                        Some(Aabb::surrounding_box(bbox.unwrap(), ab))
                    };
                    first = false;
                } else {
                    bbox = None;
                    break;
                }
            }
            bbox
        }
    }
}
