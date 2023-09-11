use std::{io::{Error, Write}, path::PathBuf, fs::{self, File}};

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::ray::Ray;

pub struct Camera {
    position: DVec3,
    focal_length: f64,
    image: Image,
}

impl Camera {
    pub fn new(position: DVec3, image: Image) -> Self {
        Self {
            position,
            focal_length: 1.0,
            image,
        }
    }

    pub fn render_image(&mut self) {

        let viewport_height = 2.0;
        let viewport_width = viewport_height * self.image.aspect_ratio();

        let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / self.image.width as f64;
        let pixel_delta_v = viewport_v / self.image.height as f64;

        let viewport_upper_left = self.position - DVec3::new(0.0, 0.0, self.focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixels = (0..self.image.height)
            .cartesian_product(0..self.image.width)
            .progress_count(self.image.width as u64 * self.image.height as u64)
            .map(|(y, x)| {

                let pixel_center = pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
                let ray_direction = pixel_center - self.position;

                let ray = Ray::new(pixel_center, ray_direction);

                (self.ray_color(ray)
                    .clamp(
                        DVec3::splat(0.0),
                        DVec3::splat(0.999)
                    ) * self.image.max_color_value as f64).into()
            })
            .collect::<Vec<Pixel>>();

        self.image.data = Some(pixels);
    }

    fn ray_color(&self, ray: Ray) -> DVec3 {
        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
    }

    pub fn save_image(&self, name: &str) -> Result<(), Error> {

        let pixels = if let Some(pixels) = &self.image.data {
            pixels.iter().map(|pixel| {
                format!(
                    "{} {} {}",
                    pixel.r,
                    pixel.g,
                    pixel.b
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
        } else {
            todo!();
        };

        let path = PathBuf::from(format!("{}/{}.ppm", crate::IMAGES_FOLDER, name));

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;

        write!(
            file,
            "{}",
            format!("P3\n{} {}\n{}\n{}\n",
                    self.image.width,
                    self.image.height,
                    self.image.max_color_value,
                    pixels
                )
            )
    }
}

pub struct Image {
    width: u32,
    height: u32,
    max_color_value: u32,
    data: Option<Vec<Pixel>>,
}

impl Image {
    pub fn from_width_height(width: u32, height: u32, max_color_value: u32) -> Self {
        Self {
            width,
            height,
            max_color_value,
            data: None,
        }
    }

    pub fn from_width_aspect_ratio(width: u32, aspect_ratio: f64, max_color_value: u32) -> Self {
        Self {
            width,
            height: (width as f64 / aspect_ratio) as u32,
            max_color_value,
            data: None,
        }
    }

    fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

struct Pixel {
    r: u32,
    g: u32,
    b: u32,
}

impl From<DVec3> for Pixel {
    fn from(value: DVec3) -> Self {
        Pixel {
            r: value.x as u32,
            g: value.y as u32,
            b: value.z as u32,
        }
    }
}