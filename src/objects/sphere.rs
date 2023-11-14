use crate::config::Point;
use crate::objects::Texture;
#[derive(Debug)]
pub struct Sphere {
    pub center_point: Point,
    pub radius: f64,
    pub texture: Texture,
}

impl Sphere {
    pub fn new(center_point: Point, radius: f64, texture: Texture) -> Self {
        Self {
            center_point,
            radius,
            texture,
        }
    }
}
