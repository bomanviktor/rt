pub mod state;

pub mod config {
    pub const PLANE_WIDTH: f64 = 1000.0;
    pub const PLANE_LENGTH: f64 = PLANE_WIDTH;
    pub const BRIGHTNESS: f64 = 100.0;
}

pub mod hittable {
    pub trait Hittable {
        fn hit(&self);
    }
}

pub mod color {
    #[derive(Debug)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        pub fn new(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b }
        }
    }
}

pub mod light_sources {
    pub use crate::color;
    pub mod ceiling;
    pub use ceiling::*;
    #[derive(Debug)]
    pub enum LightSource {
        Ceiling(Ceiling),
    }
}

pub mod raytracer {
    pub mod camera;
    pub use camera::*;
    pub mod ray;
    pub use ray::*;

    pub mod viewport;
    pub use viewport::*;
}

pub mod objects {
    pub mod cube;
    pub use cube::*;

    pub mod cylinder;
    pub use cylinder::*;

    pub mod flat_plane;
    pub use flat_plane::*;

    pub mod sphere;
    use crate::color::Color;
    pub use sphere::*;

    #[derive(Debug)]
    pub enum Object {
        Cube(Cube),
        Cylinder(Cylinder),
        FlatPlane(FlatPlane),
        Sphere(Sphere),
    }

    #[derive(Debug)]
    pub enum Texture {
        Smooth(Color),
        Metal,
        Wood,
        Glass,
        Reflective,
    }
}
#[derive(Debug)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coords {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
