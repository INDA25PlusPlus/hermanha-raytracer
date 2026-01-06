use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn mul(&self, other: &Vec3) -> Vec3 {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn scale(&self, scalar: f64) -> Vec3 {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn div(&self, scalar: f64) -> Vec3 {
        self.scale(1.0 / scalar)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalize(&self) -> Vec3 {
        self.div(self.length())
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.normalize()
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::rng();
        return Vec3::new(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        );
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::rng();
        return Vec3::new(
            rng.random_range(min..max),
            rng.random_range(min..max),
            rng.random_range(min..max),
        );
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if lensq > 1e-160 && lensq <= 1.0 {
                return p.div(lensq.sqrt());
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return on_unit_sphere.scale(-1.0);
        }
    }
}
