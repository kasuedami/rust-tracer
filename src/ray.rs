use glam::DVec3;

#[derive(Default, Debug)]
pub(crate) struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}