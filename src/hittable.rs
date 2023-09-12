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
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: Ray, point: DVec3, outward_normal: DVec3, t: f64) -> Self {

        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        Self {
            point,
            normal,
            t,
            front_face,
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