use glam::DVec3;

use crate::{ray::Ray, hittable::HitRecord};

use super::{Material, Scattered, reflect::reflect, util::random_unit_vector};

pub struct Metal {
    albedo: DVec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Self {

        let fuzz = if fuzz > 0.0 && fuzz < 1.0 {
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
        let direction = Ray::new(hit_record.point, reflected + self.fuzz * random_unit_vector());
        let attenuation = self.albedo;
        
        if direction.direction.dot(hit_record.normal) > 0.0 {
            Some(Scattered { attenuation, direction })
        } else {
            None
        }
    }
}