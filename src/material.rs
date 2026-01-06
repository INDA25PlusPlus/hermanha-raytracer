use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Copy, Debug)]

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = rec.normal.add(&Vec3::random_on_hemisphere(&rec.normal));

                if scatter_direction.length_squared() < 1e-8 {
                    *scattered = Ray::new(rec.p, rec.normal);
                } else {
                    *scattered = Ray::new(rec.p, scatter_direction)
                }

                *attenuation = *albedo;
                true
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(&r_in.direction.unit_vector(), &rec.normal);

                let fuzzed = reflected.add(&Vec3::random_unit_vector().scale(*fuzz));

                *scattered = Ray::new(rec.p, fuzzed);
                *attenuation = *albedo;

                scattered.direction.dot(&rec.normal) > 0.0
            }
        }
    }
}
