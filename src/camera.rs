use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use rand::Rng;

pub struct Camera {
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let focal_length = 1.0;
        let viewport_height = 2.0;

        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u.div(image_width as f64);
        let pixel_delta_v = viewport_v.div(image_height as f64);

        let distance_from_center = Vec3::new(0.0, 0.0, focal_length);
        let viewport_upper_left = center
            .sub(&distance_from_center)
            .sub(&viewport_v.div(2.0))
            .sub(&viewport_u.div(2.0));
        let pixel00_loc = viewport_upper_left
            .add(&pixel_delta_u.div(2.0))
            .add(&pixel_delta_v.div(2.0));

        Self {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self
            .pixel00_loc
            .add(&self.pixel_delta_u.scale(i as f64 + offset.x))
            .add(&self.pixel_delta_v.scale(j as f64 + offset.y));

        let ray_origin = self.center;
        let ray_direction = pixel_sample.sub(&ray_origin);

        Ray::new(ray_origin, ray_direction)
    }

    pub fn sample_square(&self) -> Vec3 {
        let mut rng = rand::rng();
        return Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        );
    }
}
