/// `type_aliases` is for differentiating the different `Vector3` types
pub mod type_aliases {
    pub use nalgebra::Vector3;

    /// `Point` represents a point in euclidean space
    pub type Point = Vector3<f64>;

    /// `Normal` represents the normal vector on any surface
    pub type Normal = Vector3<f64>;

    /// `Direction` represents the normalized directional vector
    pub type Direction = Vector3<f64>;

    /// `Color` represents the RGB values of a color. It is later calculated down to the u8 type
    pub type Color = Vector3<f64>;
    /// `Pixels` represents the rendered pixels in the image. It is a Vector of `Color`
    pub type Pixels = Vec<Color>;
}

pub mod color {
    use nalgebra::Vector3;

    pub trait Color {
        fn r(&self) -> u8;
        fn g(&self) -> u8;
        fn b(&self) -> u8;
        fn new(r: f64, g: f64, b: f64) -> Self;
        fn black() -> Self;
        fn white() -> Self;
        fn random() -> Self;

        fn red() -> Self;
        fn green() -> Self;
        fn blue() -> Self;
        fn yellow() -> Self;
        fn light_yellow() -> Self;
        fn correct_gamma(&self, gamma: f64) -> Self;
    }

    impl Color for Vector3<f64> {
        fn r(&self) -> u8 {
            self.x as u8
        }
        fn g(&self) -> u8 {
            self.y as u8
        }
        fn b(&self) -> u8 {
            self.z as u8
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

        fn correct_gamma(&self, gamma: f64) -> Vector3<f64> {
            let gamma_inv = 1.0 / gamma;
            // Normalize, apply gamma correction, and convert back
            let r = (self.x / 255.0).powf(gamma_inv) * 255.0;
            let g = (self.y / 255.0).powf(gamma_inv) * 255.0;
            let b = (self.z / 255.0).powf(gamma_inv) * 255.0;
            Vector3::new(r, g, b)
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
    use std::sync::Arc;
    pub mod cube;
    pub use cube::*;
    pub mod cylinder;
    pub use cylinder::*;

    pub mod flat_plane;
    pub use flat_plane::*;

    pub mod sphere;
    use crate::raytracer::Ray;
    use crate::type_aliases::{Color, Normal, Point};
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
        fn texture(&self) -> Texture;
        fn center(&self) -> Point;
        fn is_light(&self) -> bool;
    }

    pub type Objects = Vec<Arc<dyn Object>>;

    pub type Distance = f64;


    pub struct Intersection {
        pub hit_point: Point,
        pub normal: Normal,
        pub distance: Distance,
        pub texture: Texture
    }

    impl Intersection {
        pub fn new(hit_point: Point, normal: Normal, distance: Distance, texture: Texture) -> Self {
            Self {
                hit_point, normal, distance, texture
            }
        }

        pub fn color(&self) -> Color {
            match self.texture {
                Texture::Light(c) => c,
                Texture::Diffusive(c) => c,
                Texture::Glossy(c) => c,
                Texture::Reflective => Color::default(),
            }
        }

    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Texture {
        Light(Color),
        Diffusive(Color),
        Glossy(Color),
        Reflective,
    }
}
