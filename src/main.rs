use glam::DVec3;
use rust_tracer::{camera::{Image, Camera}, sphere::Sphere, world::World, hittable::Hittable};

fn main() {
    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let world = World::new(objects);

    let position = DVec3::new(0.0, 0.0, 0.0);
    let image = Image::from_width_aspect_ratio(400, 2.0, 255);
    let mut camera = Camera::new(position, 1.0, image);

    camera.render_image(&world);

    match camera.save_image("image") {
        Ok(_) => println!("Image saved successfully!"),
        Err(_) => println!("Failed to save the image!"),
    }
}
