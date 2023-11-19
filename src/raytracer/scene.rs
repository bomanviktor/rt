use crate::color::Color;
use crate::config::Point;
use crate::objects::{Objects, Sphere};
use nalgebra::Vector3;

pub struct Scene {
    pub objects: Objects,
    pub origo: Point,
}

impl Scene {
    pub fn init(_scene_data: &str) -> Self {
        let sphere1 = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, Color::new(255, 0, 0));
        let sphere2 = Sphere::new(Vector3::new(2.0, 0.0, -5.0), 1.2, Color::new(255, 0, 0));
        let sphere3 = Sphere::new(Vector3::new(-2.0, 0.0, -5.0), 1.2, Color::new(255, 0, 0));
        let sphere4 = Sphere::new(Vector3::new(0.0, -2.0, -5.0), 1.0, Color::new(255, 0, 0));
        let sphere5 = Sphere::new(Vector3::new(0.0, -4.0, -5.0), 1.0, Color::new(255, 0, 0));
        let sphere6 = Sphere::new(Vector3::new(0.0, 6.0, -5.0), 1.5, Color::new(255, 0, 0));

        let objects: Objects = vec![
            Box::new(sphere1),
            Box::new(sphere2),
            Box::new(sphere3),
            Box::new(sphere4),
            Box::new(sphere5),
            Box::new(sphere6),
            // Box::new(cylinder),
        ];

        Self {
            objects,
            origo: Point::default(),
        }
    }
}
