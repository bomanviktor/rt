use crate::config::Point;
use crate::objects::Texture;
#[derive(Debug)]
pub struct FlatPlane {
    pub center_point: Point,
    pub width: f64,
    pub length: f64,
    pub texture: Texture,
}

impl FlatPlane {
    pub fn new(center_point: Point, width: f64, length: f64, texture: Texture) -> Self {
        Self {
            center_point,
            width,
            length,
            texture,
        }
    }
}
