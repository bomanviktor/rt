use crate::color::Color;
use crate::objects::{Cube, Cylinder, FlatPlane, Objects, Sphere, Texture::*};
use nalgebra::Vector3;
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub struct Scene {
    pub objects: Objects,
}

impl Scene {
    pub fn init(_scene_data: &str) -> Self {
        let mut objects: Objects = Vec::new();

        // Create a flat plane
        let flat_plane = FlatPlane::new(Vector3::new(0.0, 1.0, 0.0), 5.0, Color::blue(), Diffusive);
        objects.push(Arc::new(flat_plane));

        // Random object generation
        for _ in 0..4 {
            let mut rng = thread_rng();
            let choose_obj = rng.gen_range(0..3); // Randomly choose the object type
            let size = rng.gen_range(0.4..=1.0);
            let pos_range = Uniform::new(-2.0, 2.0);
            let position = Vector3::new(
                pos_range.sample(&mut rng),
                rng.gen_range(-1.5..=-0.5),
                pos_range.sample(&mut rng),
            );

            match choose_obj {
                0 => objects.push(Arc::new(Sphere::new(
                    position,
                    size,
                    Color::random(),
                    Reflective,
                ))),
                1 => objects.push(Arc::new(Cube::new(
                    position,
                    size,
                    Color::random(),
                    Diffusive,
                ))),
                2 => objects.push(Arc::new(Cylinder::new(
                    position,
                    size,
                    rng.gen_range(1.0..3.0),
                    Color::random(),
                    Diffusive,
                ))),
                _ => unreachable!(),
            }
        }

        // Light source
        let light = Sphere::new(Vector3::new(-5.0, -6.0, -10.0), 2.0, Color::white(), Light);
        objects.push(Arc::new(light));

        Self { objects }
    }
}
