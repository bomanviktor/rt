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
        let sphere1 = Sphere::new(Vector3::new(3.0, 1.0, 0.0), 1.0, Glossy(RGB::navy()));

        // let flat_plane = FlatPlane::new(Vector3::new(0.0, 0.0, 0.0), 10.0, Glossy(RGB::blue()));
        let flat_plane =
            FlatPlane::new(Vector3::new(0.0, 0.0, 0.0), 10.0, Diffusive(Color::blue()));

        let light = Sphere::new(
            Vector3::new(-5.0, 6.0, -10.0),
            2.0,
            Light(RGB::light_yellow()),
        );
        //let cylinder = Cylinder::new(Vector3::new(-2.0, 0.0, -1.0), 1.0, 3.0, Glossy(RGB::green()));
        // let cylinder = Cylinder::new(Vector3::new(0.0, 0.0, 0.0), 1.0, 3.0, Reflective);
        let cylinder = Cylinder::new(Vector3::new(0.0, 0.0, 0.0), 1.0, 3.0, Glossy(RGB::green()));

        let sphere2 = Sphere::new(Vector3::new(-3.0, 1.2, 0.3), 0.2, Glossy(RGB::pink()));
        let sphere3 = Sphere::new(Vector3::new(-3.0, 0.2, 1.2), 0.2, Glossy(RGB::coral()));
        let sphere4 = Sphere::new(Vector3::new(-3.7, 0.2, 0.0), 0.2, Glossy(RGB::teal()));
        // let cube = Cube::new(Vector3::new(-3.0, 0.5, 0.0), 1.0, Reflective);
        let cube = Cube::new(Vector3::new(-3.0, 0.5, 0.0), 1.0, Diffusive(RGB::gold()));

        let objects: Objects = vec![
            Arc::new(sphere1),
            Arc::new(sphere2),
            Arc::new(sphere3),
            Arc::new(sphere4),
            Arc::new(cylinder),
            Arc::new(flat_plane),
            Arc::new(cube),
            Arc::new(light),
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
