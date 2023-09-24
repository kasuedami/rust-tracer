use std::sync::Arc;

use rand::Rng;
use rust_tracer::{prelude::*, material::util::random_unit_vector};

pub fn create_world() -> World {
    let material_ground = Arc::new(Lambertian::new(DVec3::new(0.5, 0.5, 0.5)));
    let material_dielectric = Arc::new(Dielectric::new(1.5));
    let material_lambertian = Arc::new(Lambertian::new(DVec3::new(0.4, 0.2, 0.1)));
    let material_metal = Arc::new(Metal::new(DVec3::new(0.7, 0.6, 0.5), 0.0));

    let mut objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::stationary(
            DVec3::new(0.0, -1000., 0.0),
            1000.0,
            material_ground,
        )),
        Box::new(Sphere::stationary(
            DVec3::new(0.0, 1.0, 0.0),
            1.0,
            material_dielectric.clone(),
        )),
        Box::new(Sphere::stationary(
            DVec3::new(-4.0, 1.0, 0.0),
            1.0,
            material_lambertian,
        )),
        Box::new(Sphere::stationary(
            DVec3::new(4.0, 1.0, 0.0),
            1.0,
            material_metal,
        )),
    ];

    let mut rand_thread = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let position = DVec3::new(
                a as f64 + 0.9 * rand_thread.gen_range(0.0..1.1),
                0.2,
                b as f64 + 0.9 * rand_thread.gen_range(0.0..1.0),
            );

            if position.distance(DVec3::new(4.0, 0.2, 0.0)) > 0.9 {
                let choose_material = rand_thread.gen_range(0.0..1.0);

                let sphere = if choose_material < 0.8 {
                    let material = Arc::new(Lambertian::new(random_unit_vector()));
                    let direction = DVec3::new(0.0, rand_thread.gen_range(0.0..0.5), 0.0);
                    Box::new(Sphere::moving(position, direction, 0.2, material))
                } else if choose_material < 0.95 {
                    let material = Arc::new(Metal::new(
                        random_unit_vector(),
                        rand_thread.gen_range(0.0..0.5),
                    ));
                    Box::new(Sphere::stationary(position, 0.2, material))
                } else {
                    Box::new(Sphere::stationary(
                        position,
                        0.2,
                        material_dielectric.clone(),
                    ))
                };

                objects.push(sphere);
            }
        }
    }

    World::new(objects)
}

pub fn create_camera(world: &World) {
    let look_from = DVec3::new(13.0, 2.0, 3.0);
    let image = Image::from_width_aspect_ratio(400, 16.0 / 9.0, 255);

    let mut camera = CameraBuilder::default()
        .look_from(look_from)
        .look_at(DVec3::ZERO)
        .fov(20.0)
        .defocus_angle(0.6)
        .image(image)
        .build();

    camera.render_image(world)
}