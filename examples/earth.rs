use std::{path::Path, sync::Arc};

use rust_tracer::prelude::*;

fn main() {
    let earth_texture_path = Path::new("assets/earthmap.jpg");
    let earth_texture = Texture::Image(TextureImage::from_file(earth_texture_path));
    // let earth_material = Arc::new(Lambertian::with_solid(DVec3::X));
    let earth_material = Arc::new(Lambertian::new(earth_texture));
    let earth_sphere = Sphere::stationary(DVec3::ZERO, 2.0, earth_material);

    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);
    let look_from = DVec3::new(0.0, 0.0, 12.0);

    let mut camera = CameraBuilder::default()
        .fov(20.0)
        .look_from(look_from)
        .look_at(DVec3::ZERO)
        .image(image)
        .build();

    camera.render_image_with_progress(&earth_sphere);

    match camera.save_image("earth") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}