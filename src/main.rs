mod camera;
mod hittable;
mod hittable_list;
mod image_handler;
mod interval;
mod material;
mod plane;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use image_handler::ImageHandler;
use vec3::{Point3, Vec3};

use crate::{material::Material, vec3::Color};

use rand::Rng;

fn main() {
    // Camera setup
    let mut camera = Camera::new(400, 16.0 / 9.0);
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    // Image handler
    let mut image_handler = ImageHandler::new(camera.image_width, camera.image_height());

    // World

    let mut world = HittableList::new();
    world.add_plane(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        },
    );
    world.add_sphere(
        Point3::new(0.0, 0.0, -10.0),
        0.5,
        Material::Lambertian {
            albedo: Color::new(0.4, 0.4, 0.6),
        },
    );
    world.add_sphere(
        Point3::new(-0.8, -0.2, -10.0),
        0.3,
        Material::Metal {
            albedo: Color::new(0.8, 0.8, -5.0),
            fuzz: 0.3,
        },
    );

    // Render
    camera.render(&world, &mut image_handler);
    image_handler.save("image.png");
}
