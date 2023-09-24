use std::sync::Arc;

use rust_tracer::prelude::*;

pub fn create_camera(fov: f64, world: &HittableList) {
    let mut camera = camera(fov);
    camera.render_image(world);
}

pub fn create_world() -> HittableList {
    let material_ground = Arc::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(DVec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.0));

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::stationary(
            DVec3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::stationary(
            DVec3::new(0.0, 0.0, -1.0),
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

    HittableList::new(objects)
}

fn camera(fov: f64) -> Camera {
    let look_from = DVec3::new(-2.0, 2.0, 1.0);
    let look_at = DVec3::new(0.0, 0.0, -1.0);
    let focus_dist = look_from.distance(look_at);
    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);

    CameraBuilder::default()
        .look_from(look_from)
        .look_at(look_at)
        .fov(fov)
        .focus_dist(focus_dist)
        .image(image)
        .build()
}
