use crate::hittable::HitRecord;
use crate::hittable_list::HittableList;
use crate::image_handler::ImageHandler;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use rand::Rng;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel: 10,
            max_depth: 50,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        };
        camera.initialize();
        camera
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u.div(self.image_width as f64);
        self.pixel_delta_v = viewport_v.div(self.image_height as f64);

        let viewport_upper_left = self
            .center
            .sub(&Vec3::new(0.0, 0.0, focal_length))
            .sub(&viewport_u.scale(0.5))
            .sub(&viewport_v.scale(0.5));
        self.pixel00_loc = viewport_upper_left
            .add(&self.pixel_delta_u.scale(0.5))
            .add(&self.pixel_delta_v.scale(0.5));
    }

    pub fn render(&mut self, world: &HittableList, image_handler: &mut ImageHandler) {
        self.initialize();

        for y in (0..self.image_height).rev() {
            for x in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color = pixel_color.add(&self.ray_color(&ray, self.max_depth, world));
                }

                pixel_color = pixel_color.scale(self.pixel_samples_scale);

                let r = (pixel_color.x.clamp(0.0, 1.0) * 255.999) as u8;
                let g = (pixel_color.y.clamp(0.0, 1.0) * 255.999) as u8;
                let b = (pixel_color.z.clamp(0.0, 1.0) * 255.999) as u8;
                image_handler.set_pixel(x, y, r, g, b);
            }
        }
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(
            ray,
            Interval {
                min: 0.001,
                max: f64::INFINITY,
            },
            &mut rec,
        ) {
            let direction = rec.normal.add(&Vec3::random_on_hemisphere(&rec.normal));
            return self
                .ray_color(&Ray::new(rec.p, direction), depth - 1, world)
                .scale(0.5);
        }

        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        let start_color = Color::new(1.0, 1.0, 1.0);
        let end_color = Color::new(0.5, 0.7, 1.0);
        start_color.scale(1.0 - a).add(&end_color.scale(a))
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self
            .pixel00_loc
            .add(&self.pixel_delta_u.scale(i as f64 + offset.x))
            .add(&self.pixel_delta_v.scale(j as f64 + offset.y));

        let ray_origin = self.center;
        let ray_direction = pixel_sample.sub(&ray_origin);

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        )
    }

    pub fn image_height(&self) -> u32 {
        self.image_height
    }
}
