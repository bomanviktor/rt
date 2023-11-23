pub mod config {
    use crate::color::Color;
    pub use nalgebra::Vector3;

    pub type Point = Vector3<f64>;
    pub type Pixels = Vec<Vec<Color>>;
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

pub mod gui {
    pub mod interface;
    pub use interface::*;
}

pub mod raytracer {
    pub mod camera;
    pub use camera::*;
    pub mod ray;
    pub use ray::*;

    pub mod scene;
    pub use scene::*;

    pub type Resolution = (u32, u32);
}

pub mod objects {
    pub mod cube;

    pub use cube::*;
    use nalgebra::Vector3;

    pub mod cylinder;
    pub use cylinder::*;

    pub mod flat_plane;
    pub use flat_plane::*;

    pub mod sphere;
    use crate::color::Color;
    use crate::config::Point;
    use crate::raytracer::Ray;
    pub use sphere::*;

    type Distance = f64;

    /// Type alias for `Option<(Vector3<f64>, f64)>`
    pub type Intersection = Option<(Point, Distance)>;

    /// [Discriminant equation](https://en.wikipedia.org/wiki/Discriminant)
    ///
    /// Returns `None` if `b² - 4ac < 0.0`
    pub fn discriminant(a: f64, b: f64, c: f64) -> Option<f64> {
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant >= 0.0 {
            Some(discriminant)
        } else {
            None
        }
    }

    pub trait Object {
        fn intersection(&self, ray: &Ray) -> Option<(Vector3<f64>, f64)>;
        fn normal_at(&self, point: Vector3<f64>) -> Vector3<f64>;
        fn color(&self) -> Color;
    }

    pub type Objects = Vec<Box<dyn Object>>;

    #[derive(Debug)]
    pub enum Texture {
        Smooth(Color),
        Metal,
        Wood,
        Glass,
        Reflective,
    }
}
