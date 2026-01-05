use image::{ImageBuffer, Rgb, RgbImage};

pub struct ImageHandler {
    image: RgbImage,
}

impl ImageHandler {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: ImageBuffer::new(width, height),
        }
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        self.image.put_pixel(x, y, Rgb([r, g, b]));
    }
    pub fn save(&self, path: &str) {
        self.image.save(path).unwrap();
    }
}
