pub mod image_writer {
    pub fn write(width: i32, height: i32) {

        println!("P3\n{} {}\n255", width, height);

        for j in 0..height {
            for i in 0..width {
                let r = i as f32 / (width - 1) as f32;
                let g = j as f32 / (height - 1) as f32;
                let b = 0.0;

                let int_r = (255.999 * r) as i32;
                let int_g = (255.999 * g) as i32;
                let int_b = (255.999 * b) as i32;

                println!("{} {} {}", int_r, int_g, int_b);
            }
        }
    }
}