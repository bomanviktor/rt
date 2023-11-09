use crate::color::Color;
#[derive(Debug)]
pub struct Ceiling {
    pub color: Color,
    pub brightness: u8,
}

impl Ceiling {
    pub fn new(color: Color, brightness: u8) -> Self {
        Self { color, brightness }
    }
}
