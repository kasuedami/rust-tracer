use std::{ops::Range, sync::Arc};

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};
use glam::DVec3;

pub struct Sphere {
    start_position: DVec3,
    end_position: DVec3,
    moving: bool,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn stationary(
        position: DVec3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            start_position: position,
            end_position: position,
            moving: false,
            radius,
            material,
        }
    }

    pub fn moving(
        start_position: DVec3,
        end_position: DVec3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            start_position,
            end_position,
            moving: true,
            radius,
            material,
        }
    }

    fn current_position(&self, time: f64) -> DVec3 {
        self.start_position + self.end_position * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let position = if self.moving {
            self.current_position(ray.time)
        } else {
            self.start_position
        };

        let oc = ray.origin - position;
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
        let outward_normal = (point - position) / self.radius;

        Some(HitRecord::new(
            ray,
            point,
            outward_normal,
            root,
            self.material.clone(),
        ))
    }
}
