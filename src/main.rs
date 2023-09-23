use std::sync::Arc;

use glam::DVec3;
use rust_tracer::{
    camera::{builder::CameraBuilder, Image},
    hittable::Hittable,
    material::{dialectric::Dialectric, lambertian::Lambertian, metal::Metal},
    sphere::Sphere,
    world::World,
};

fn main() {
    let material_ground = Arc::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(DVec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dialectric::new(1.5));
    let material_right = Arc::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.0));

    let objects: Vec<Box<dyn Hittable + Sync + Send>> = vec![
        Box::new(Sphere::stationary(
            DVec3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::moving(
            DVec3::new(0.0, 0.0, -1.0),
            DVec3::new(0.0, 1.0, 0.0),
            0.5,
            material_center,
        )),
        Box::new(Sphere::stationary(
            DVec3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )),
        Box::new(Sphere::stationary(
            DVec3::new(-1.0, 0.0, -1.0),
            -0.4,
            material_left,
        )),
        Box::new(Sphere::stationary(
            DVec3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ];

    let world = World::new(objects);

    let look_from = DVec3::new(-2.0, 2.0, 1.0);
    let look_at = DVec3::new(0.0, 0.0, -1.0);
    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);

    let mut camera = CameraBuilder::default()
        .look_from(look_from)
        .look_at(look_at)
        .defocus_angle(10.0)
        .focus_dist(3.4)
        .image(image)
        .build();

    camera.render_image(&world);

    match camera.save_image("image") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
