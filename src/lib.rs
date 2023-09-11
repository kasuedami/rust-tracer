pub mod image_writer {
    use std::{path::Path, fs::File, io::{Write, Error}};

    use indicatif::ProgressBar;

    pub fn write(width: u32, height: u32, path: &Path) -> Result<(), Error> {

        let mut ppm_content = String::new();
        ppm_content.push_str(&format!("P3\n{} {}\n255", width, height));

        let bar = ProgressBar::new(width as u64 * height as u64);

        for j in 0..height {
            for i in 0..width {
                let r = i as f32 / (width - 1) as f32;
                let g = j as f32 / (height - 1) as f32;
                let b = 0.0;

                let int_r = (255.999 * r) as i32;
                let int_g = (255.999 * g) as i32;
                let int_b = (255.999 * b) as i32;

                ppm_content.push_str(&format!("{} {} {}", int_r, int_g, int_b));
                bar.inc(1);
            }
        }

        bar.finish();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;
        write!(file, "{}", ppm_content)?;

        Ok(())
    }
}