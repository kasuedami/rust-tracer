use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::{util::random_unit_vector, Material, Scattered};

pub struct Lambertian {
    albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit_record: HitRecord) -> Option<Scattered> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.abs_diff_eq(DVec3::ZERO, 1e-8) {
            scatter_direction = hit_record.normal;
        }

        let direction = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;

        Some(Scattered {
            attenuation,
            direction,
        })
    }
}
