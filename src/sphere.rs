use std::{ops::Range, sync::Arc};

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};
use glam::DVec3;

pub struct Sphere {
    position: DVec3,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(position: DVec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            position,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin - self.position;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if !t_range.contains(&root) {
            root = (-half_b + sqrtd) / a;

            if !t_range.contains(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.position) / self.radius;

        Some(HitRecord::new(
            ray,
            point,
            outward_normal,
            root,
            self.material.clone(),
        ))
    }
}
