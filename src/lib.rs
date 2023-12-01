pub mod config {
    pub use nalgebra::Vector3;

    pub type Point = Vector3<f64>;
    pub type Pixels = Vec<Vector3<f64>>;
}

pub mod color {
    use nalgebra::Vector3;

    pub trait Color {
        fn r(&self) -> f64;
        fn g(&self) -> f64;
        fn b(&self) -> f64;
        fn new(r: f64, g: f64, b: f64) -> Self;
        fn black() -> Self;
        fn white() -> Self;
        fn random() -> Self;

        fn red() -> Self;
        fn green() -> Self;
        fn blue() -> Self;
        fn yellow() -> Self;
        fn light_yellow() -> Self;
        fn apply_gamma_correction(&self, gamma: f64) -> Vector3<u8>;
    }

    impl Color for Vector3<f64> {
        fn r(&self) -> f64 {
            self.x
        }
        fn g(&self) -> f64 {
            self.y
        }
        fn b(&self) -> f64 {
            self.z
        }

        fn new(r: f64, g: f64, b: f64) -> Self {
            Self::new(r, g, b)
        }

        fn black() -> Self {
            Self::new(0.0, 0.0, 0.0)
        }
        fn white() -> Self {
            Self::new(255., 255., 255.)
        }
        fn random() -> Self {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            Self::new(
                rng.gen_range(0.0..255.),
                rng.gen_range(0.0..255.),
                rng.gen_range(0.0..255.),
            )
        }

        /// #### r: 255, g: 0, b: 0
        fn red() -> Self {
            Self::new(255., 0., 0.)
        }

        /// #### r: 0, g: 255, b: 0
        fn green() -> Self {
            Self::new(0., 255., 0.)
        }

        /// #### r: 0, g: 0, b: 255
        fn blue() -> Self {
            Self::new(0., 0., 255.)
        }

        fn yellow() -> Self {
            Self::new(255., 255., 0.)
        }

        fn light_yellow() -> Self {
            Self::new(255., 255., 224.)
        }

        fn apply_gamma_correction(&self, gamma: f64) -> Vector3<u8> {
            let gamma_inv = 1.0 / gamma;

            // Normalize, apply gamma correction, and convert back
            let r = (self.x / 255.0).powf(gamma_inv) * 255.0;
            let g = (self.y / 255.0).powf(gamma_inv) * 255.0;
            let b = (self.z / 255.0).powf(gamma_inv) * 255.0;
            Vector3::new(r as u8, g as u8, b as u8)
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
        fn color(&self) -> Vector3<f64>;
        fn texture(&self) -> Texture;
        fn center(&self) -> Point;
        fn is_light(&self) -> bool;
    }

    pub type Objects = Vec<Arc<dyn Object>>;

    pub type Distance = f64;
    /// Type alias for `Option<(Vector3<f64>, f64)>`
    pub type Intersection = Option<(Point, Distance)>;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Texture {
        Diffusive,
        Reflective,
        Glossy,
        Light,
    }
}
