use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *&outward_normal.scale(-1.0);
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
