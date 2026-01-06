use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

pub enum HittableObject {
    Sphere(Sphere),
    Plane(Plane),
}

pub struct HittableList {
    objects: Vec<HittableObject>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_plane(&mut self, point: Point3, normal: Vec3) {
        let plane = Plane::new(point, normal);
        self.objects.push(HittableObject::Plane(plane));
    }

    pub fn add_sphere(&mut self, center: Point3, radius: f64) {
        let sphere = Sphere::new(center, radius);
        self.objects.push(HittableObject::Sphere(sphere));
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_rec = HitRecord::new();

        for object in &self.objects {
            let hit = match object {
                HittableObject::Sphere(s) => s.hit(
                    ray,
                    Interval {
                        min: ray_t.min,
                        max: closest_so_far,
                    },
                    &mut temp_rec,
                ),
                HittableObject::Plane(p) => p.hit(
                    ray,
                    Interval {
                        min: ray_t.min,
                        max: closest_so_far,
                    },
                    &mut temp_rec,
                ),
            };

            if hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }
}
