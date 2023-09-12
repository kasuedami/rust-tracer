use std::ops::Range;

use glam::DVec3;

use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

pub struct HitRecord {
    point: DVec3,
    normal: DVec3,
    t: f64,
}

impl HitRecord {
    pub fn new(point: DVec3, normal: DVec3, t: f64) -> Self {
        Self {
            point,
            normal,
            t
        }
    }

    pub fn point(&self) -> DVec3 {
        self.point
    }

    pub fn normal(&self) -> DVec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}