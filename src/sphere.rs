use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center.sub(&ray.origin);
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let dis_sqrt = discriminant.sqrt();

        let mut root = (h - dis_sqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + dis_sqrt) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.normal = rec.p.sub(&self.center).div(self.radius);

        let outward_normal = rec.p.sub(&self.center).div(self.radius);
        rec.set_face_normal(ray, &outward_normal);

        return true;
    }
}
