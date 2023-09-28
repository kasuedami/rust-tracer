use glam::{DVec2, DVec3};

#[derive(Debug)]
pub enum Texture {
    Solid(DVec3),
    Checker(Checker),
}

impl Texture {
    pub fn sample(&self, uv: DVec2, position: DVec3) -> DVec3 {
        match self {
            Texture::Solid(color) => *color,
            Texture::Checker(checker) => checker.sample(uv, position),
        }
    }
}

#[derive(Debug)]
pub struct Checker {
    scale: f64,
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Box<Texture>, odd: Box<Texture>) -> Self {
        Self { scale, even, odd }
    }

    pub fn with_solid(scale: f64, even: DVec3, odd: DVec3) -> Self {
        Self {
            scale,
            even: Texture::Solid(even).into(),
            odd: Texture::Solid(odd).into(),
        }
    }

    fn sample(&self, uv: DVec2, position: DVec3) -> DVec3 {
        let scale_inverse = self.scale.recip();
        let x = (position.x * scale_inverse).floor() as i64;
        let y = (position.y * scale_inverse).floor() as i64;
        let z = (position.z * scale_inverse).floor() as i64;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.sample(uv, position)
        } else {
            self.odd.sample(uv, position)
        }
    }
}
