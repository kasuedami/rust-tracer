use std::{io::{Error, Write}, path::PathBuf, fs::{self, File}};

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;

pub struct Camera {
    position: DVec3,
    image: Image,
}

impl Camera {
    pub fn new(position: DVec3, image: Image) -> Self {
        Self {
            position,
            image,
        }
    }

    pub fn render_image(&mut self) {
        let pixels = (0..self.image.height)
            .cartesian_product(0..self.image.width)
            .progress_count(self.image.width as u64 * self.image.height as u64)
            .map(|(y, x)| {

                let color = DVec3::new(
                    x as f64 / (self.image.width - 1) as f64,
                    y as f64 / (self.image.height - 1) as f64,
                    0.0)
                    .clamp(
                        DVec3::splat(0.0),
                        DVec3::splat(0.999)
                    ) * self.image.max_color_value as f64;

                
                color.into()
            })
            .collect::<Vec<Pixel>>();

        self.image.data = Some(pixels);
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

    fn aspect_ratio(&self) -> u64 {
        self.width as u64 / self.height as u64
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