use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

pub mod dialectric;
pub mod lambertian;
pub mod metal;
pub mod util;

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<Scattered>;
}

pub struct Scattered {
    pub attenuation: DVec3,
    pub direction: Ray,
}
