use std::{rc::Rc, f64::consts::PI};

use glam::DVec3;
use rust_tracer::{
    camera::{Camera, Image},
    hittable::Hittable,
    material::lambertian::Lambertian,
    sphere::Sphere,
    world::World,
};

fn main() {

    let r = (PI / 4.0).cos();

    let material_left = Rc::new(Lambertian::new(DVec3::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(DVec3::new(1.0, 0.0, 0.0)));

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(DVec3::new(-r, 0.0, -1.0), r, material_left)),
        Box::new(Sphere::new(DVec3::new( r, 0.0, -1.0), r, material_right)),
    ];

    let world = World::new(objects);

    let position = DVec3::new(0.0, 0.0, 0.0);
    let look_at = DVec3::new(0.0, 0.0, -1.0);

    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);
    let mut camera = Camera::new(position, look_at, DVec3::Y, 90.0, 100, 50, image);

    camera.render_image(&world);

    match camera.save_image("touching_spheres") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
