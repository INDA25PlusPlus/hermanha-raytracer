mod camera;
mod hittable;
mod hittable_list;
mod image_handler;
mod interval;
mod plane;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use image_handler::ImageHandler;
use vec3::{Point3, Vec3};

fn main() {
    // Camera setup
    let mut camera = Camera::new(400, 16.0 / 9.0);
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    // Image handler
    let mut image_handler = ImageHandler::new(camera.image_width, camera.image_height());

    // World
    let mut world = HittableList::new();
    world.add_plane(Point3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0));
    world.add_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);

    // Render
    camera.render(&world, &mut image_handler);
    image_handler.save("image.png");
}
