use std::{ops::Range, sync::Arc};

use crate::{
    hittable::{AxisAlignedBoundingBox, HitRecord, Hittable},
    material::Material,
    ray::Ray,
};
use glam::{DVec2, DVec3};

#[derive(Debug)]
pub struct Sphere {
    start_position: DVec3,
    direction: DVec3,
    moving: bool,
    radius: f64,
    bounding_box: AxisAlignedBoundingBox,
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
            direction: DVec3::ZERO,
            moving: false,
            radius,
            bounding_box: AxisAlignedBoundingBox::from_corners(
                position - radius,
                position + radius,
            ),
            material,
        }
    }

    pub fn moving(
        start_position: DVec3,
        direction: DVec3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let box0 =
            AxisAlignedBoundingBox::from_corners(start_position - radius, start_position + radius);
        let box1 = AxisAlignedBoundingBox::from_corners(
            (start_position + direction) - radius,
            (start_position + direction) + radius,
        );

        Self {
            start_position,
            direction,
            moving: true,
            radius,
            bounding_box: AxisAlignedBoundingBox::from_boxes(box0, box1),
            material,
        }
    }

    fn current_position(&self, time: f64) -> DVec3 {
        self.start_position + self.direction * time
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
            DVec2::ZERO,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }
}
