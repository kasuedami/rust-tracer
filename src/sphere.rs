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

    pub(crate) fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.orign() - self.position;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}