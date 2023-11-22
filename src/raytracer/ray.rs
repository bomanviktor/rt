use crate::color::Color;
use crate::config::Point;
use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
    pub collisions: Vec<Color>,
    pub hit_light_source: bool,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self {
            origin,
            direction,
            collisions: Vec::new(),
            hit_light_source: false,
        }
    }
}
