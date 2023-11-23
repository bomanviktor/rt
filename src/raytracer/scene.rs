use crate::color::Color;
use crate::config::Point;
use crate::objects::{Cube, Cylinder, FlatPlane, Objects, Sphere, Texture::*};
use nalgebra::Vector3;

pub struct Scene {
    pub objects: Objects,
    pub light_sources: Objects,
    pub origo: Point,
}

impl Scene {
    pub fn init(_scene_data: &str) -> Self {
        let sphere1 = Sphere::new(Vector3::new(0.0, -1.0, -5.0), 1.0, Color::red(), Diffusive);
        // let sphere2 = Sphere::new(Vector3::new(2.0, 0.0, -5.0), 1.2, Color::new(255, 0, 0));
        // let sphere3 = Sphere::new(Vector3::new(-2.0, 0.0, -5.0), 1.2, Color::new(255, 0, 0));
        // let sphere4 = Sphere::new(Vector3::new(0.0, -2.0, -5.0), 1.0, Color::new(255, 0, 0));
        // let sphere5 = Sphere::new(Vector3::new(0.0, -4.0, -5.0), 1.0, Color::new(255, 0, 0));
        // let sphere6 = Sphere::new(Vector3::new(0.0, 6.0, -5.0), 1.5, Color::new(255, 0, 0));
        let cylinder = Cylinder::new(
            Vector3::new(2.0, -4.0, -5.0),
            1.0,
            4.0,
            Color::green(),
            Diffusive,
        );

        let flat_plane =
            FlatPlane::new(Vector3::new(0.0, 0.0, -5.0), 5.0, Color::blue(), Diffusive);
        let light = Sphere::new(
            Vector3::new(-4.0, -7.0, 0.0),
            2.0,
            Color::light_yellow(),
            Light,
        );

        let cube = Cube::new(
            Vector3::new(-2.0, -1.0, -5.0),
            1.0,
            Color::new(0, 0, 255),
            Diffusive,
        );

        let objects: Objects = vec![
            Box::new(sphere1),
            // Box::new(sphere2),
            // Box::new(sphere3),
            // Box::new(sphere4),
            // Box::new(sphere5),
            // Box::new(sphere6),
            Box::new(cylinder),
            Box::new(flat_plane),
            Box::new(cube),
            Box::new(light.clone()),
        ];

        let light_sources: Objects = vec![Box::new(light)];

        Self {
            objects,
            light_sources,
            origo: Point::default(),
        }
    }
}
