use crate::Coords;
#[derive(Debug)]
pub struct Camera {
    pub ray_amount: u64,
    pub position: Coords,
    pub direction: Direction,
}

impl Camera {
    pub fn new(ray_amount: u64, position: Coords, direction: Direction) -> Self {
        Self {
            ray_amount,
            position,
            direction,
        }
    }
}
// Keep track of the center point where the camera will face
#[derive(Debug)]
pub struct Direction {
    pub x: f64,
    pub y: f64,
}

impl Direction {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
