use crate::objects::Texture;
use crate::Coords;
#[derive(Debug)]
pub struct Cylinder {
    pub coords: Coords,
    pub radius: f64,
    pub height: f64,
    pub texture: Texture,
}

impl Cylinder {
    pub fn new(coords: Coords, radius: f64, height: f64, texture: Texture) -> Self {
        Self {
            coords,
            radius,
            height,
            texture,
        }
    }
}
