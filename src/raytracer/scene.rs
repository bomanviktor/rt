use std::sync::Arc;

use crate::color::RGB;
use crate::objects::{Cube, FlatPlane, Objects, Sphere};
use crate::textures::Texture::*;
use crate::type_aliases::{Color, Point};

pub struct Scene {
    pub objects: Objects,
    pub brightness: f64,
}

impl Scene {
    pub fn init(brightness: f64) -> Self {
        let flat_plane = FlatPlane::new(Point::new(0.0, 0.0, 0.0), 10.0, Diffusive(RGB::blue()));
        let light = Sphere::new(Point::new(0.0, 5.0, 0.0), 1.0, Light(RGB::white()));
        let sphere = Sphere::new(Point::new(0.0, 0.5, 0.0), 0.5, Diffusive(RGB::green()));
        let sphere2 = Sphere::new(Point::new(1.0, 0.2, 4.0), 0.2, Diffusive(RGB::green()));
        let cube = Cube::new(Point::new(-2.0, 1.0, -2.0), 2.0, Reflective);
        let cube2 = Cube::new(Point::new(-2.0, 2.25, -2.0), 0.5, Light(RGB::orange()));

        let objects: Objects = vec![
            Arc::new(flat_plane),
            Arc::new(light),
            Arc::new(cube),
            Arc::new(cube2),
            Arc::new(sphere),
            Arc::new(sphere2),
        ];

        // Adjust invalid value in brightness
        let brightness = if brightness > 1.0 { 1.0 } else { brightness };

        Self {
            objects,
            brightness: if brightness <= 0.0 {
                0.0001
            } else {
                brightness
            },
        }
    }

    pub fn background(&self) -> Color {
        Color::white() * self.brightness
    }
}
