use std::sync::Arc;

use rust_tracer::prelude::*;

fn main() {
    let checker_texture = Texture::Checker(Checker::with_solid(
        0.8,
        DVec3::new(0.2, 0.3, 0.1),
        DVec3::new(0.9, 0.9, 0.9),
    ));

    let checker_material = Arc::new(Lambertian::new(checker_texture));

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::stationary(
            DVec3::new(0.0, -10.0, 0.0),
            10.0,
            checker_material.clone(),
        )),
        Box::new(Sphere::stationary(
            DVec3::new(0.0, 10.0, 0.0),
            10.0,
            checker_material,
        )),
    ];

    let world = BoundingVolumeHierarchyNode::new(objects);

    let look_from = DVec3::new(13.0, 2.0, 3.0);
    let look_at = DVec3::new(0.0, 0.0, 0.0);
    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);

    let mut camera = CameraBuilder::default()
        .look_from(look_from)
        .look_at(look_at)
        .fov(20.0)
        .image(image)
        .build();

    camera.render_image_with_progress(&world);

    match camera.save_image("checker_spheres") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
