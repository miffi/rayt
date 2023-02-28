use crate::hittable as h;
use crate::ray::Ray;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn h::Hittable>>,
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
    fn hit(&self, r: &Ray, (t_min, t_max): (f64, f64)) -> Option<h::HitRecord> {
        let f = |(closest, rec), obj: &Rc<dyn h::Hittable>| {
            if let Some(temp_rec) = obj.hit(r, (t_min, closest)) {
                (temp_rec.t, Some(temp_rec))
            } else {
                (closest, rec)
            }
        };

        self.objects.iter().fold((t_max, None), f).1
    }
}
