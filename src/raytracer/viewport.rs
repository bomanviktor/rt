use crate::color::Color;
use crate::config::Point;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct ViewPort {
    pub width: f64,
    pub height: f64,
    pub distance: f64,
    pub fov: f64,
    pub pixels: Vec<Vec<Color>>,
}

impl ViewPort {
    pub fn new(width: f64, height: f64, distance: f64) -> Self {
        Self {
            width,
            height,
            distance,
            fov: field_of_view(width, distance),
            pixels: Vec::with_capacity(height as usize),
        }
    }

    pub fn top_left(&self) -> Point {
        let x = 0.0 - self.width / 2.0;
        let y = 0.0 - self.height / 2.0;
        let z = 0.0;
        Point::new(x, y, z)
    }
    pub fn top_right(&self) -> Point {
        let x = 0.0 + self.width / 2.0;
        let y = 0.0 - self.height / 2.0;
        let z = 0.0;
        Point::new(x, y, z)
    }

    pub fn bottom_left(&self) -> Point {
        let x = 0.0 - self.width / 2.0;
        let y = 0.0 + self.height / 2.0;
        let z = 0.0;
        Point::new(x, y, z)
    }
    pub fn bottom_right(&self) -> Point {
        let x = 0.0 + self.width / 2.0;
        let y = 0.0 + self.height / 2.0;
        let z = 0.0;
        Point::new(x, y, z)
    }
}

fn field_of_view(width: f64, distance: f64) -> f64 {
    let radians = 2.0 * (width / 2.0 * distance).atan();
    (radians * 180.0) / PI as f64 // Return in degrees
}
