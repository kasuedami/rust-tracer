use glam::DVec3;

#[derive(Default, Debug, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_with_time(origin: DVec3, direction: DVec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}
