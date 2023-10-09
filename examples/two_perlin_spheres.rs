use std::sync::Arc;

use rust_tracer::prelude::*;

fn main() {
    let perlin_material = Arc::new(Lambertian::new(Texture::Perlin(PerlinTexture::new_scaled(4.0))));
    let sphere0 = Box::new(Sphere::stationary(
        DVec3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    ));
    let sphere1 = Box::new(Sphere::stationary(
        DVec3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    ));

    let hittable_list = HittableList::new(vec![sphere0, sphere1]);

    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);
    let look_from = DVec3::new(13.0, 2.0, 3.0);

    let mut camera = CameraBuilder::default()
        .fov(20.0)
        .look_from(look_from)
        .look_at(DVec3::ZERO)
        .image(image)
        .build();

    camera.render_image_with_progress(&hittable_list);

    match camera.save_image("two_perlin_spheres") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
