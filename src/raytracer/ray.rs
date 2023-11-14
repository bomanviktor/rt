use crate::color::Color;
use crate::config::{Pixel, Point};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
    pub pixel_to_render: Pixel,
    pub collisions: Vec<Color>,
}

impl Ray {
    pub fn new(origin: Point, direction: Point, pixel_to_render: Pixel) -> Self {
        Self {
            origin,
            direction,
            pixel_to_render,
            collisions: Vec::new(),
        }
    }
}
