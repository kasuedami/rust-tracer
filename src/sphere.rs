use glam::DVec3;
use crate::ray::Ray;

pub struct Sphere {
    position: DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(position: DVec3, radius: f64) -> Self {
        Self {
            position,
            radius,
        }
    }

    pub(crate) fn hit(&self, ray: &Ray) -> f64 {
        let oc = ray.orign() - self.position;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}