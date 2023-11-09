use crate::objects::Texture;
use crate::Coords;
#[derive(Debug)]
pub struct FlatPlane {
    pub position: Coords,
    pub width: f64,
    pub length: f64,
    pub texture: Texture,
}

impl FlatPlane {
    pub fn new(position: Coords, width: f64, length: f64, texture: Texture) -> Self {
        Self {
            position,
            width,
            length,
            texture,
        }
    }
}
