use std::rc::Rc;

use crate::{hittable, ray};

pub struct HittableList {
    pub objects: Vec<Rc<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Rc<dyn hittable::Hittable>) {
        self.objects.push(object)
    }
}

impl hittable::Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let mut closest_so_far = t_max;
        let mut acc = None;

        for object in &self.objects {
            if let Some(record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t();
                acc = Some(record);
            }
        }

        acc
    }
}
