use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct World {
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Hittable + Sync + Send>>) -> Self {
        Self { objects }
    }

    pub fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_range.end;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_range.start..closest_so_far) {
                closest_so_far = hit.t;

                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
