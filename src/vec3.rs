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
}
