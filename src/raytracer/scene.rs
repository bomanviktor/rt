use std::sync::Arc;

use crate::color::RGB;
use crate::objects::{Cube, Cylinder, FlatPlane, Objects, Sphere};
use crate::textures::Texture::*;
use crate::type_aliases::{Color, Point};
use nalgebra::Vector3;

pub struct Scene {
    pub objects: Objects,
    pub brightness: f64,
}

impl Scene {
    pub fn init(brightness: f64) -> Self {
        let flat_plane = FlatPlane::new(Point::new(0.0, 0.0, 0.0), 10.0, Diffusive(RGB::grey()));

        // AROUND CYLINDER LIGHT
        let cylinder = Cylinder::new(Point::new(0.0, 0.0, 0.0), 1.0, 2.0, Light(RGB::coral()));
        let sphere1 = Sphere::new(
            Vector3::new(3.0, 1.0, 0.0),
            1.0,
            Diffusive(Color::mint_green()),
        );
        let sphere2 = Sphere::new(Point::new(-3.0, 1.2, 0.0), 0.2, Glossy(RGB::pink()));
        let sphere3 = Sphere::new(Point::new(-3.0, 0.2, 1.2), 0.2, Glossy(RGB::coral()));
        let sphere4 = Sphere::new(Point::new(0.0, 2.5, 0.0), 0.5, Glossy(RGB::gold()));
        let sphere5 = Sphere::new(Point::new(-1.5, 0.5, -1.0), 0.5, Glossy(RGB::teal()));
        let cube = Cube::new(Point::new(-3.0, 0.5, 0.0), 1.0, Reflective);

        // AROUND SPHERE LIGHT
        let sphere_light = Sphere::new(Point::new(4.0, 0.8, 5.0), 0.8, Light(RGB::mint_green()));
        let beige_sphere = Sphere::new(Point::new(5.0, 0.5, 7.0), 0.5, Glossy(RGB::beige()));
        let coral_sphere = Sphere::new(Point::new(3.0, 0.4, 5.0), 0.4, Diffusive(RGB::beige()));

        // AROUND CUBE LIGHT
        let cube_light = Cube::new(Point::new(-4.0, 0.5, -6.0), 1.0, Light(RGB::cyan()));
        let cube_sphere1 = Sphere::new(Point::new(-4.0, 0.2, -4.0), 0.2, Diffusive(RGB::green()));
        let cube_sphere2 = Sphere::new(Point::new(-3.0, 0.3, -5.0), 0.3, Glossy(RGB::orange()));
        let cube_sphere3 = Sphere::new(Point::new(-5.5, 0.4, -6.0), 0.4, Glossy(RGB::silver()));

        // AROUND FLAT LIGHT
        let flat_light = FlatPlane::new(Point::new(6.0, 0.0001, -6.0), 1.0, Light(RGB::magenta()));
        let flat_sphere = Sphere::new(Point::new(6.0, 1.0, -6.0), 0.5, Glossy(RGB::white()));

        // HATE AIK SPHERES
        let yellow = Sphere::new(Point::new(-5.5, 0.3, 6.0), 0.3, Light(RGB::yellow()));
        let red = Sphere::new(Point::new(-4.0, 0.3, 7.5), 0.3, Light(RGB::red()));
        let blue = Sphere::new(Point::new(-3.0, 0.3, 6.0), 0.3, Light(RGB::blue()));

        let objects: Objects = vec![
            Arc::new(flat_plane),
            Arc::new(cylinder),
            Arc::new(sphere1),
            Arc::new(sphere2),
            Arc::new(sphere3),
            Arc::new(sphere4),
            Arc::new(sphere5),
            Arc::new(cube),
            Arc::new(sphere_light),
            Arc::new(beige_sphere),
            Arc::new(coral_sphere),
            Arc::new(cube_light),
            Arc::new(cube_sphere1),
            Arc::new(cube_sphere2),
            Arc::new(cube_sphere3),
            Arc::new(flat_light),
            Arc::new(flat_sphere),
            Arc::new(red),
            Arc::new(yellow),
            Arc::new(blue),
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
