use std::rc::Rc;

use glam::DVec3;
use rust_tracer::{
    camera::{Camera, Image},
    hittable::Hittable,
    material::{dialectric::Dialectric, lambertian::Lambertian, metal::Metal},
    sphere::Sphere,
    world::World,
};

fn main() {
    let material_ground = Rc::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(DVec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dialectric::new(1.5));
    let material_right = Rc::new(Metal::new(DVec3::new(0.8, 0.6, 0.2), 0.0));

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(
            DVec3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(
            DVec3::new(0.0, 0.0, -1.0),
            0.5,
            material_center,
        )),
        Box::new(Sphere::new(
            DVec3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )),
        Box::new(Sphere::new(
            DVec3::new(-1.0, 0.0, -1.0),
            -0.4,
            material_left,
        )),
        Box::new(Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
    ];

    let world = World::new(objects);

    let position = DVec3::new(-2.0, 2.0, 1.0);
    let look_at = DVec3::new(0.0, 0.0, -1.0);
    let distance = position.distance(look_at);

    let distant_image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);
    let zoom_image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);
    
    let mut distant_camera = Camera::new(position, look_at, DVec3::Y, 90.0, 0.0, distance, 100, 50, distant_image);
    let mut zoom_camera = Camera::new(position, look_at, DVec3::Y, 20.0, 0.0, distance, 100, 50, zoom_image);

    distant_camera.render_image(&world);

    match distant_camera.save_image("distant_view") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }

    zoom_camera.render_image(&world);

    match zoom_camera.save_image("zoom_view") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}