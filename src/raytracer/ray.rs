use crate::color::Color;
use crate::config::Point;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
    pub collisions: Vec<Color>,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self {
            origin,
            direction,
            collisions: Vec::new(),
        }
    }
}
