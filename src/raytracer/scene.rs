use std::sync::Arc;

use crate::color::RGB;
use crate::objects::*;
use crate::textures::Texture::*;
use crate::type_aliases::{Color, Point};

pub struct Scene {
    pub objects: Objects,
    pub brightness: f64,
}

impl Scene {
    pub fn init(brightness: f64) -> Self {
        let flat_plane = FlatPlane::new(
            Point::new(0.0, 0.0, 0.0),
            10.0,
            Diffusive(RGB::light_blue()),
        );
        let light = Cylinder::new(Point::default(), 1.0, 2.0, Diffusive(RGB::orange()));
        let sphere = Sphere::new(Point::new(3.0, 1.0, 0.0), 1.0, Diffusive(RGB::mint_green()));
        let cube = Cube::new(Point::new(-3.0, 0.5, 0.0), 1.0, Diffusive(RGB::coral()));

        let objects: Objects = vec![
            Arc::new(flat_plane),
            Arc::new(light),
            Arc::new(sphere),
            Arc::new(cube),
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
