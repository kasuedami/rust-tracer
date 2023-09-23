use glam::DVec3;

use super::{Camera, Image};

pub struct CameraBuilder {
    look_from: DVec3,
    look_at: DVec3,
    up: DVec3,
    fov: f64,
    defocus_angle: f64,
    focus_dist: f64,
    samples_per_pixel: u32,
    max_depth: u32,
    image: Image,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            look_from: DVec3::NEG_Z,
            look_at: DVec3::ZERO,
            up: DVec3::Y,
            fov: 90.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
            samples_per_pixel: 100,
            max_depth: 50,
            image: Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255),
        }
    }
}

impl CameraBuilder {

    pub fn look_from(mut self, look_from: DVec3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: DVec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn up(mut self, up: DVec3) -> Self {
        self.up = up;
        self
    }

    pub fn fov(mut self, fov: f64) -> Self {
        self.fov = fov;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn image(mut self, image: Image) -> Self {
        self.image = image;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.look_from,
            self.look_at,
            self.up,
            self.fov,
            self.defocus_angle,
            self.focus_dist,
            self.samples_per_pixel,
            self.max_depth,
            self.image,
        )
    }
}
