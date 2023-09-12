use glam::DVec3;

use crate::{ray::Ray, hittable::HitRecord};

use super::{Material, Scattered, reflect::reflect};

pub struct Metal {
    albedo: DVec3,
}

impl Metal {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<Scattered> {
        let reflected = reflect(ray.direction.normalize(), hit_record.normal);
        let direction = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        
        Some(Scattered { attenuation, direction })
    }
}