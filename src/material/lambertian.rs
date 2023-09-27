use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::{util::random_unit_vector, Material, Scattered};

#[derive(Debug)]
pub struct Lambertian {
    albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<Scattered> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.abs_diff_eq(DVec3::ZERO, 1e-8) {
            scatter_direction = hit_record.normal;
        }

        let direction = Ray::new_with_time(hit_record.point, scatter_direction, ray.time);
        let attenuation = self.albedo;

        Some(Scattered {
            attenuation,
            direction,
        })
    }
}
