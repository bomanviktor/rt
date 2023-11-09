use crate::color::Color;

#[derive(Debug)]
pub struct ViewPort {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>,
}
