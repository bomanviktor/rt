use std::sync::Arc;

use crate::color::RGB;
use crate::objects::{Cube, Cylinder, FlatPlane, Objects, Sphere};
use crate::textures::Texture::*;
use crate::type_aliases::Color;
use nalgebra::Vector3;

pub struct Scene {
    pub objects: Objects,
    pub brightness: f64,
}

impl Scene {
    pub fn init(_scene_data: &str, brightness: f64) -> Self {
        let sphere1 = Sphere::new(Vector3::new(3.0, -1.0, 0.0), 1.0, Reflective);
        let sphere2 = Sphere::new(Vector3::new(3.0, -0.5, 2.0), 0.5, Glossy(RGB::pink()));

        let cylinder = Cylinder::new(Vector3::new(0.0, -3.0, 0.0), 1.0, 3.0, Glossy(RGB::green()));

        let flat_plane = FlatPlane::new(Vector3::new(0.0, 0.0, 0.0), 10.0, Glossy(RGB::blue()));

        let light = Sphere::new(
            Vector3::new(-5.0, -6.0, -10.0),
            2.0,
            Light(RGB::light_yellow()),
        );

        let cube = Cube::new(Vector3::new(-2.0, -0.5, 0.0), 1.0, Glossy(RGB::yellow()));

        let objects: Objects = vec![
            Arc::new(sphere1),
            Arc::new(sphere2),
            Arc::new(cylinder),
            Arc::new(flat_plane),
            Arc::new(cube),
            Arc::new(light),
        ];

        Self {
            objects,
            brightness: if brightness >= 0.0 {
                brightness
            } else {
                0.0001
            },
        }
    }

    pub fn background(&self) -> Color {
        Color::white() * self.brightness
    }
}
