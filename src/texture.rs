use std::path::Path;

use glam::{DVec2, DVec3};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};

pub(crate) trait Sample {
    fn sample(&self, uv: DVec2, position: DVec3) -> DVec3;
}

#[derive(Debug)]
pub enum Texture {
    Solid(DVec3),
    Checker(Checker),
    Image(TextureImage),
}

impl Sample for Texture {
    fn sample(&self, uv: DVec2, position: DVec3) -> DVec3 {
        match self {
            Texture::Solid(color) => *color,
            Texture::Checker(checker) => checker.sample(uv, position),
            Texture::Image(image) => image.sample(uv, position),
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
}

impl Sample for Checker {
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

#[derive(Debug)]
pub struct TextureImage {
    data: DynamicImage,
}

impl TextureImage {
    pub fn from_file(file: &Path) -> Self {
        let data = ImageReader::open(file).unwrap().decode().unwrap();
        Self { data }
    }
}

impl Sample for TextureImage {
    fn sample(&self, uv: DVec2, _position: DVec3) -> DVec3 {

        let pixel_space_u = (uv.x.clamp(0.0, 1.0) * self.data.width() as f64) as u32;
        let pixel_space_v = ((1.0 - uv.y.clamp(0.0, 1.0)) * self.data.height() as f64) as u32;

        // dbg!((pixel_space_u, pixel_space_v));

        let pixel = self.data.get_pixel(pixel_space_u, pixel_space_v);

        let scaler = 255.0_f64.recip();

        DVec3::new(pixel.0[0] as f64, pixel.0[1] as f64, pixel.0[2] as f64) * scaler
    }
}
