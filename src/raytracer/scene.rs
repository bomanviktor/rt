use rand::Rng;
use std::sync::Arc; // Assuming you have the rand crate in your dependencies

use crate::color::Color;
use crate::config::Point;
use crate::objects::{Cube, Objects, Sphere, Texture::*};
use nalgebra::Vector3;

pub struct Scene {
    pub objects: Objects,
    pub light_sources: Objects,
    pub origo: Point,
}

impl Scene {
    pub fn init(_scene_data: &str) -> Self {
        let mut rng = rand::thread_rng();
        let mut objects: Objects = Vec::new();

        // Adding a flat plane
        //let flat_plane = FlatPlane::new(Vector3::new(0.0, 4.0, 0.0), 5.0, Color::blue(), Diffusive);
        // objects.push(Arc::new(flat_plane));

        // Adding random objects
        for _ in 0..6 {
            let x = rng.gen_range(0.0..5.0);
            let y = rng.gen_range(0.0..5.0);
            let z = rng.gen_range(0.0..5.0);
            let size = rng.gen_range(0.5..1.5);

            match rng.gen_range(0..2) {
                0 => {
                    // Sphere
                    let sphere =
                        Sphere::new(Vector3::new(x, y, z), size, Color::random(), Diffusive);
                    objects.push(Arc::new(sphere));
                }
                1 => {
                    // Cube
                    let cube = Cube::new(Vector3::new(x, y, z), size, Color::random(), Diffusive);
                    objects.push(Arc::new(cube));
                }
                _ => {}
            }
        }

        // Adding a light source
        let light = Sphere::new(Vector3::new(-4.0, 4.0, -8.0), 2.0, Color::white(), Light);
        objects.push(Arc::new(light.clone()));

        let light_sources: Objects = vec![Arc::new(light)];

        Self {
            objects,
            light_sources,
            origo: Point::default(),
        }
    }
}
