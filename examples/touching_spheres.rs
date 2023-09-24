use std::{f64::consts::PI, sync::Arc};

use rust_tracer::prelude::*;

fn main() {
    let r = (PI / 4.0).cos();

    let material_left = Arc::new(Lambertian::new(DVec3::new(0.0, 0.0, 1.0)));
    let material_right = Arc::new(Lambertian::new(DVec3::new(1.0, 0.0, 0.0)));

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::stationary(
            DVec3::new(-r, 0.0, -1.0),
            r,
            material_left,
        )),
        Box::new(Sphere::stationary(
            DVec3::new(r, 0.0, -1.0),
            r,
            material_right,
        )),
    ];

    let world = HittableList::new(objects);

    let look_from = DVec3::new(0.0, 0.0, 0.0);
    let look_at = DVec3::new(0.0, 0.0, -1.0);
    let focus_dist = look_from.distance(look_at);
    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);

    let mut camera = CameraBuilder::default()
        .look_from(look_from)
        .look_at(look_at)
        .focus_dist(focus_dist)
        .image(image)
        .build();

    camera.render_image_with_progress(&world);

    match camera.save_image("touching_spheres") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
