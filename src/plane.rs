use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Plane {
    point: Point3,
    normal: Vec3,
    mat: Material,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, mat: Material) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            mat,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = ray.direction.dot(&self.normal);

        if denom.abs() < 1e-6 {
            return false;
        }

        let t = self.point.sub(&ray.origin).dot(&self.normal) / denom;

        if !ray_t.surrounds(t) {
            return false;
        }

        rec.t = t;
        rec.p = ray.at(rec.t);

        rec.set_face_normal(ray, &self.normal);
        rec.mat = Some(self.mat);

        true
    }
}
