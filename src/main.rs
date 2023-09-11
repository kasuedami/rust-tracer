use glam::DVec3;
use rust_tracer::camera::{Image, Camera};

fn main() {
    let position = DVec3::new(0.0, 0.0, 0.0);
    let image = Image::from_width_aspect_ratio(800, 2.0, 256);
    let mut camera = Camera::new(position, 1.0, image);

    camera.render_image();

    match camera.save_image("image") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
