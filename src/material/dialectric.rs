use glam::DVec3;

use crate::{hittable::HitRecord, material::util::refract, ray::Ray};

use super::{util::reflect, Material, Scattered};

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
        let attenuation = DVec3::splat(1.0);
        let refraction_ratio = if hit_record.front_face {
            self.refraction_index.recip()
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction.dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let refracted = if cannot_refract {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        let direction = Ray::new_with_time(hit_record.point, refracted, ray.time);

        Some(Scattered {
            attenuation,
            direction,
        })
    }
}
