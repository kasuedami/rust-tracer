use std::{ops::Range, rc::Rc};

use glam::DVec3;

use crate::{material::Material, ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub material: Rc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        ray: Ray,
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            material,
            front_face,
        }
    }
}
