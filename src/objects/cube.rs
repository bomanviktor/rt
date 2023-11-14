use crate::config::Point;
use crate::objects::Texture;
#[derive(Debug)]
pub struct Cube {
    pub center_point: Point,
    pub size: f64,
    pub texture: Texture,
}

impl Cube {
    pub fn new(center_point: Point, size: f64, texture: Texture) -> Self {
        Self {
            center_point,
            size,
            texture,
        }
    }
}
