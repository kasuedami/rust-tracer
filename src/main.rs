use std::rc::Rc;

use glam::DVec3;
use rust_tracer::{camera::{Image, Camera}, sphere::Sphere, world::World, hittable::Hittable, material::{lambertian::Lambertian, metal::Metal}};

fn main() {

    let material_ground = Rc::new(Lambertian::new(DVec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(DVec3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(DVec3::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(DVec3::new(0.8, 0.6, 0.2)));

    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0, material_ground)),
        Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0),    0.5,   material_center)),
        Box::new(Sphere::new(DVec3::new(-1.0, 0.0, -1.0),   0.5,   material_left)),
        Box::new(Sphere::new(DVec3::new(1.0, 0.0, -1.0), 0.5,   material_right)),
    ];

    let world = World::new(objects);

    let position = DVec3::new(0.0, 0.0, 0.0);
    let image = Image::from_width_aspect_ratio(400, 2.0, 255);
    let mut camera = Camera::new(position, 1.0, 50, 10, image);

    camera.render_image(&world);

    match camera.save_image("image") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
