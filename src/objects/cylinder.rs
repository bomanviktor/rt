use crate::config::Point;
use crate::objects::Texture;
#[derive(Debug)]
pub struct Cylinder {
    pub center_point: Point,
    pub radius: f64,
    pub height: f64,
    pub texture: Texture,
}

impl Cylinder {
    pub fn new(center_point: Point, radius: f64, height: f64, texture: Texture) -> Self {
        Self {
            center_point,
            radius,
            height,
            texture,
        }
    }
}
