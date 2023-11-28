pub mod config {
    use crate::color::Color;
    pub use nalgebra::Vector3;

    pub type Point = Vector3<f64>;
    pub type Pixels = Vec<Vec<Color>>;
}

pub mod color {
    #[derive(Debug, Clone, Copy)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        pub fn new(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b }
        }
        pub fn black() -> Self {
            Self { r: 0, g: 0, b: 0 }
        }

        pub fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            Self {
                r: rng.gen_range(0..255),
                g: rng.gen_range(0..255),
                b: rng.gen_range(0..255),
            }
        }

        pub fn white() -> Self {
            Self {
                r: 255,
                g: 255,
                b: 255,
            }
        }

        /// #### r: 255, g: 0, b: 0
        pub fn red() -> Self {
            Self { r: 255, g: 0, b: 0 }
        }
        /// #### r: 0, g: 255, b: 0
        pub fn green() -> Self {
            Self { r: 0, g: 255, b: 0 }
        }
        /// #### r: 0, g: 0, b: 255
        pub fn blue() -> Self {
            Self { r: 0, g: 0, b: 255 }
        }

        pub fn yellow() -> Self {
            Self {
                r: 255,
                g: 255,
                b: 0,
            }
        }

        pub fn light_yellow() -> Self {
            Self {
                r: 255,
                g: 255,
                b: 224,
            }
        }
    }

    impl Default for Color {
        fn default() -> Self {
            Color::black()
        }
    }
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

    use std::sync::Arc;

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

    pub trait Object: Send + Sync {
        fn intersection(&self, ray: &Ray) -> Intersection;
        fn normal_at(&self, ray: &Ray, point: Vector3<f64>) -> Vector3<f64>;
        fn color(&self) -> Color;
        fn texture(&self) -> Texture;
    }

    pub type Objects = Vec<Arc<dyn Object>>;

    pub type Distance = f64;
    /// Type alias for `Option<(Vector3<f64>, f64)>`
    pub type Intersection = Option<(Point, Distance)>;

    #[derive(Debug, Clone, Copy)]
    pub enum Texture {
        Diffusive,
        Reflective,
        Glossy,
        Light,
    }
}
