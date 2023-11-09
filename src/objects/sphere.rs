use crate::objects::Texture;
use crate::Coords;
#[derive(Debug)]
pub struct Sphere {
    pub coords: Coords,
    pub radius: f64,
    pub texture: Texture,
}

impl Sphere {
    pub fn new(coords: Coords, radius: f64, texture: Texture) -> Self {
        Self {
            coords,
            radius,
            texture,
        }
    }
}
