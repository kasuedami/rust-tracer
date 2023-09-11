use std::path::Path;

use rust_tracer::*;

fn main() {
    let path = Path::new("images/image.ppm");

    match image_writer::write(256, 256, &path) {
        Ok(_) => println!("Image was created!"),
        Err(_) => println!("Writing image failed!"),
    }
}
