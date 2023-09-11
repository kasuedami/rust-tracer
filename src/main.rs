use glam::DVec3;
use rust_tracer::{camera::{Image, Camera}, sphere::Sphere};

fn main() {
    let objects = vec![
        Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(DVec3::new(0.5, 0.5, -1.0), 0.2),
        Sphere::new(DVec3::new(-0.5, 0.5, -1.0), 0.2),
    ];

    let position = DVec3::new(0.0, 0.0, 0.0);
    let image = Image::from_width_aspect_ratio(800, 2.0, 256);
    let mut camera = Camera::new(position, 0.5, image);

    camera.render_image(&objects);

    match camera.save_image("image") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
