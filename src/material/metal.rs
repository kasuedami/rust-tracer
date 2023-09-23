use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::{util::random_unit_vector, util::reflect, Material, Scattered};

pub struct Metal {
    albedo: DVec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Self {
        let fuzz = if (0.0..1.0).contains(&fuzz) {
            fuzz
        } else {
            1.0
        };

        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<Scattered> {
        let reflected = reflect(ray.direction.normalize(), hit_record.normal);
        let direction = Ray::new_with_time(
            hit_record.point,
            reflected + self.fuzz * random_unit_vector(),
            ray.time,
        );
        let attenuation = self.albedo;

        if direction.direction.dot(hit_record.normal) > 0.0 {
            Some(Scattered {
                attenuation,
                direction,
            })
        } else {
            None
        }
    }
}
