use crate::objects::Texture;
use crate::Coords;
#[derive(Debug)]
pub struct Cube {
    pub coords: Coords,
    pub size: f64,
    pub texture: Texture,
}

impl Cube {
    pub fn new(coords: Coords, size: f64, texture: Texture) -> Self {
        Self {
            coords,
            size,
            texture,
        }
    }
}
