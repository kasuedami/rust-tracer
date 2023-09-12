use glam::DVec3;

use crate::{ray::Ray, hittable::HitRecord, material::util::refract};

use super::{Material, Scattered};

pub struct Dialectric {
    refraction_index: f64,
}

impl Dialectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<Scattered> {
        let attenuation = DVec3::ONE;
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let refracted = refract(unit_direction, hit_record.normal, refraction_ratio);
        let direction = Ray::new(hit_record.point, refracted);

        Some(Scattered { attenuation, direction })
    }
}