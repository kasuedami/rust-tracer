use glam::DVec3;

use crate::{ray::Ray, hittable::HitRecord};

pub mod lambertian;
pub mod metal;
pub mod reflect;

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<Scattered>;
}

pub struct Scattered {
    pub attenuation: DVec3,
    pub direction: Ray,
}

pub mod util {
    use glam::DVec3;
    use rand::Rng;

    pub fn random_in_unit_sphere() -> DVec3 {
        let mut rng = rand::thread_rng();
        loop {
            let vec = DVec3::new(
                rng.gen_range(-1.0..1.),
                rng.gen_range(-1.0..1.),
                rng.gen_range(-1.0..1.),
            );
    
            if vec.length_squared() < 1. {
                break vec;
            }
        }
    }

    pub fn random_unit_vector() -> DVec3 {
        return random_in_unit_sphere().normalize();
    }

    pub fn random_on_hemisphere(normal: DVec3) -> DVec3 {
        let on_unit_sphere = random_in_unit_sphere();

        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}