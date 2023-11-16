pub mod state;

pub mod config {
    use nalgebra::Vector2;
    pub use nalgebra::Vector3;
    pub type Point = Vector3<f64>;

    pub type Pixel = Vector2<i64>;
}

pub mod hittable {
    pub trait Hittable {
        fn hit(&self);
    }
}

pub mod color {
    #[derive(Debug, Clone)]
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

    impl Default for Color {
        fn default() -> Self {
            Self {
                r: 169,
                g: 169,
                b: 169,
            }
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
