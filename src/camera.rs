use std::{
    fs::{self, File},
    io::{Error, Write},
    path::PathBuf,
};

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::Rng;

use crate::{ray::Ray, world::World, material::util::random_in_unit_sphere};

pub struct Camera {
    position: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    defocus_angle: f64,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
    pixel00_loc: DVec3,
    samples_per_pixel: u32,
    max_depth: u32,
    image: Image,
}

impl Camera {
    pub fn new( 
        look_from: DVec3,
        look_at: DVec3,
        up: DVec3,
        fov: f64,
        defocus_angle: f64,
        focus_dist: f64,
        samples_per_pixel: u32,
        max_depth: u32,
        image: Image,
    ) -> Self {

        let position = look_from;
        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * image.aspect_ratio();

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image.width as f64;
        let pixel_delta_v = viewport_v / image.height as f64;

        let viewport_upper_left =
            position - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            position,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            pixel00_loc,
            samples_per_pixel,
            max_depth,
            image,
        }
    }

    pub fn render_image(&mut self, world: &World) {
        let pixels = (0..self.image.height)
            .cartesian_product(0..self.image.width)
            .progress_count(self.image.width as u64 * self.image.height as u64)
            .map(|(y, x)| {
                let mut color = DVec3::ZERO;

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += self.ray_color(ray, self.max_depth, world);
                }

                let normalized = color * (1.0 / self.samples_per_pixel as f64);
                let gamma = Self::linear_to_gamma(normalized);

                (gamma.clamp(DVec3::splat(0.0), DVec3::splat(0.999))
                    * self.image.max_color_value as f64)
                    .into()
            })
            .collect::<Vec<Pixel>>();

        self.image.data = Some(pixels);
    }

    fn ray_color(&self, ray: Ray, depth: u32, world: &World) -> DVec3 {
        if depth == 0 {
            return DVec3::ZERO;
        }

        if let Some(hit) = world.hit(ray, 0.001..f64::INFINITY) {
            if let Some(scattered) = hit.material.clone().scatter(ray, hit) {
                return scattered.attenuation
                    * self.ray_color(scattered.direction, depth - 1, &world);
            } else {
                return DVec3::ZERO;
            }
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> DVec3 {
        let px = -0.5 + rand::thread_rng().gen_range(0.0..1.0);
        let py = -0.5 + rand::thread_rng().gen_range(0.0..1.0);

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn linear_to_gamma(color: DVec3) -> DVec3 {
        DVec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt())
    }

    fn defocus_disk_sample(&self) -> DVec3 {
        let p = random_in_unit_sphere();
        
        self.position + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    pub fn save_image(&self, name: &str) -> Result<(), Error> {
        let pixels = if let Some(pixels) = &self.image.data {
            pixels
                .iter()
                .map(|pixel| format!("{} {} {}", pixel.r, pixel.g, pixel.b))
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
            format!(
                "P3\n{} {}\n{}\n{}\n",
                self.image.width, self.image.height, self.image.max_color_value, pixels
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
