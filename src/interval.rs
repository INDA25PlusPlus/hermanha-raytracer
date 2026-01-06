pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }
}
