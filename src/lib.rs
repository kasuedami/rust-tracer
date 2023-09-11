const COLOR_RESOLUTION: u32 = 255;

pub mod image_writer {
    use super::COLOR_RESOLUTION;

    use std::{path::Path, fs::File, io::{Write, Error}};

    use glam::DVec3;
    use indicatif::ProgressIterator;
    use itertools::Itertools;

    pub fn write(width: u32, height: u32, path: &Path) -> Result<(), Error> {

        let pixels = (0..height)
            .cartesian_product(0..width)
            .progress_count(width as u64 * height as u64)
            .map(|(y, x)| {

                let color = DVec3::new(
                    x as f64 / (width - 1) as f64,
                    y as f64 / (height - 1) as f64,
                    0.0)
                    .clamp(
                        DVec3::splat(0.0),
                        DVec3::splat(0.999)
                    ) * 256.0;
                
                format!(
                    "{} {} {}",
                    color.x, color.y, color.z
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;
        write!(
            file,
            "{}",
            format!(
                "P3
{} {}
{}
{pixels}
",
                width,
                height,
                COLOR_RESOLUTION
            )
        )?;

        Ok(())
    }
}