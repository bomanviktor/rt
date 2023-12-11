/// Dependencies and constants for modules
pub mod config {
    /// Configurations for `rays.rs`
    pub mod rays {
        pub use crate::objects::Intersection;
        pub use crate::raytracer::Scene;
        pub use crate::type_aliases::{Color, Direction, Normal, Point};
        pub use nalgebra::Vector3;
        pub use rand::Rng;

        pub const MAX_DEPTH: u8 = 50;
    }

    /// Configurations for `camera.rs`
    pub mod camera {
        pub use crate::color::RGB;
        pub use crate::raytracer::{Ray, Scene};
        pub use crate::type_aliases::{Pixels, Point, Resolution};
        pub use nalgebra::Vector3;
        pub use rand::Rng;
        pub use rayon::prelude::*;
        pub use std::io::Write;
        pub use std::sync::Arc;

        pub const DEFAULT_CAMERA_POSITION: Point = Point::new(1.0, 0.5, 0.0);
        pub const DEFAULT_SAMPLE_SIZE: u16 = 1000;
        pub const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
        pub const DEFAULT_SENSOR_WIDTH: f64 = 1.0;
        pub const DEFAULT_RESOLUTION: Resolution = (800, 600);
    }
}

/// `type_aliases` is for differentiating the different `Vector3` types
pub mod type_aliases {
    pub use nalgebra::Vector3;

    /// `Point` represents a point in euclidean space
    pub type Point = Vector3<f64>;

    /// `Normal` represents the normal vector on any surface
    pub type Normal = Vector3<f64>;

    /// `Direction` represents the normalized directional vector
    pub type Direction = Vector3<f64>;

    /// Directions are to _further_ abstract for the non math homies.
    /// Can be used in `Normal` and `Direction`.
    pub trait Directions {
        fn up() -> Normal;
        fn down() -> Normal;

        fn near_zero(&self) -> bool;
    }

    impl Directions for Vector3<f64> {
        fn up() -> Normal {
            Normal::new(0.0, 1.0, 0.0)
        }

        fn down() -> Normal {
            Normal::new(0.0, -1.0, 0.0)
        }

        /// Catch degenerate directions
        fn near_zero(&self) -> bool {
            let s = 1e-8;
            self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
        }
    }

    /// `Color` represents the RGB values of a color. It is later calculated down to the u8 type
    pub type Color = Vector3<f64>;

    /// `Pixels` represents the rendered pixels in the image. It is a Vector of `Color`
    pub type Pixels = Vec<Color>;

    pub type Resolution = (u32, u32);
}

pub mod color {
    use crate::raytracer::Scene;
    use nalgebra::Vector3;

    /// `Color` is a utility-trait for the `Vector3` type.
    ///
    /// It allows for defining preset colors that could easily be called
    /// on the `Vector3` type.
    pub trait RGB {
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
        fn light_blue() -> Self;
        fn indigo() -> Self;
        fn yellow() -> Self;
        fn light_yellow() -> Self;
        fn grey() -> Self;
        fn pink() -> Self;
        fn cyan() -> Self;
        fn orange() -> Self;
        fn brown() -> Self;
        fn purple() -> Self;
        fn lavender() -> Self;
        fn magenta() -> Self;
        fn violet() -> Self;
        fn maroon() -> Self;
        fn olive() -> Self;
        fn navy() -> Self;
        fn teal() -> Self;
        fn peach() -> Self;
        fn gold() -> Self;
        fn silver() -> Self;
        fn beige() -> Self;
        fn turquoise() -> Self;
        fn coral() -> Self;
        fn mint_green() -> Self;
        fn sky_blue() -> Self;

        fn correct_gamma(&self, gamma: f64) -> Self;
    }

    impl RGB for Vector3<f64> {
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

        fn red() -> Self {
            Self::new(255., 0., 0.)
        }

        fn green() -> Self {
            Self::new(0., 255., 0.)
        }

        fn blue() -> Self {
            Self::new(0., 0., 255.)
        }
        fn light_blue() -> Self {
            Self::new(135., 206., 250.)
        }
        fn indigo() -> Self {
            Self::new(75., 0., 130.)
        }

        fn yellow() -> Self {
            Self::new(255., 255., 0.)
        }

        fn light_yellow() -> Self {
            Self::new(255., 255., 224.)
        }

        fn grey() -> Self {
            Self::new(169., 169., 169.)
        }
        fn pink() -> Self {
            Self::new(255., 20., 147.)
        }
        fn cyan() -> Self {
            Self::new(0., 255., 255.)
        }
        fn orange() -> Self {
            Self::new(255., 165., 0.)
        }
        fn brown() -> Self {
            Self::new(165., 42., 42.)
        }
        fn purple() -> Self {
            Self::new(128., 0., 128.)
        }

        fn lavender() -> Self {
            Self::new(230., 230., 250.)
        }

        fn magenta() -> Self {
            Self::new(255., 0., 255.)
        }

        fn violet() -> Self {
            Self::new(238., 130., 238.)
        }

        fn maroon() -> Self {
            Self::new(128., 0., 0.)
        }

        fn olive() -> Self {
            Self::new(128., 128., 0.)
        }

        fn navy() -> Self {
            Self::new(0., 0., 128.)
        }

        fn teal() -> Self {
            Self::new(0., 128., 128.)
        }

        fn peach() -> Self {
            Self::new(255., 218., 185.)
        }

        fn gold() -> Self {
            Self::new(255., 215., 0.)
        }

        fn silver() -> Self {
            Self::new(192., 192., 192.)
        }

        fn beige() -> Self {
            Self::new(245., 245., 220.)
        }

        fn turquoise() -> Self {
            Self::new(64., 224., 208.)
        }

        fn coral() -> Self {
            Self::new(255., 127., 80.)
        }

        fn mint_green() -> Self {
            Self::new(152., 251., 152.)
        }

        fn sky_blue() -> Self {
            Self::new(135., 206., 235.)
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

    impl crate::config::camera::Ray {
        /// Calculates the average color of the pixel using following calculation:
        ///
        /// 1. Assign the light source that the ray hit as the  
        pub fn average_color(&mut self, scene: &Scene) -> crate::type_aliases::Color {
            if self.collisions.len() == 1 {
                return self.collisions[0];
            }

            let mut total = if self.hit_light_source {
                self.collisions.pop().unwrap()
            } else {
                scene.background() / 5.0
            };

            if total.x <= 5.0 {
                total.x = 5.0;
            }

            if total.y <= 5.0 {
                total.y = 5.0;
            }

            if total.z <= 5.0 {
                total.z = 5.0;
            }

            self.collisions.iter().rev().for_each(|color| {
                let r = if color.x <= 10.0 {
                    5.0 / 255.
                } else {
                    color.x / 255.
                };

                let g = if color.y <= 10.0 {
                    5.0 / 255.
                } else {
                    color.y / 255.
                };

                let b = if color.z <= 10.0 {
                    5.0 / 255.
                } else {
                    color.z / 255.
                };

                total.x *= r;
                total.y *= g;
                total.z *= b;
            });

            total
        }
    }
}

pub mod gui {
    pub use gdk_pixbuf::Pixbuf;
    pub use glib::clone;
    pub use glib::signal::Inhibit;
    pub use gtk::{prelude::*, Image};
    pub use gtk::{
        Box as GtkBox, Button, ComboBoxText, CssProvider, Entry, FlowBox, Orientation, Scale,
        Separator, Window, WindowType,
    };
    pub use nalgebra::Vector3;
    pub use std::cell::RefCell;
    pub use std::rc::Rc;
    pub use std::sync::Arc;

    pub struct AppState {
        pub spheres: Vec<SphereConfig>,
        pub cylinders: Vec<CylinderConfig>,
        pub cubes: Vec<CubeConfig>,
        pub flat_planes: Vec<FlatPlaneConfig>,
        pub brightness: f64,
    }

    pub struct SphereConfig {
        pub id: Rc<RefCell<u32>>,
        pub pos_x_entry: Rc<RefCell<Entry>>,
        pub pos_y_entry: Rc<RefCell<Entry>>,
        pub pos_z_entry: Rc<RefCell<Entry>>,
        pub radius_entry: Rc<RefCell<Entry>>,
        pub material_selector: Rc<RefCell<ComboBoxText>>,
        pub color_button: Rc<RefCell<gtk::ColorButton>>,
    }
    #[derive(Clone)]
    pub struct CylinderConfig {
        pub id: Rc<RefCell<u32>>,
        pub pos_x_entry: Rc<RefCell<Entry>>,
        pub pos_y_entry: Rc<RefCell<Entry>>,
        pub pos_z_entry: Rc<RefCell<Entry>>,
        pub radius_entry: Rc<RefCell<Entry>>,
        pub material_selector: Rc<RefCell<ComboBoxText>>,
        pub height_entry: Rc<RefCell<Entry>>,
        pub color_button: Rc<RefCell<gtk::ColorButton>>,
    }

    pub struct CubeConfig {
        pub id: Rc<RefCell<u32>>,
        pub pos_x_entry: Rc<RefCell<Entry>>,
        pub pos_y_entry: Rc<RefCell<Entry>>,
        pub pos_z_entry: Rc<RefCell<Entry>>,
        pub radius_entry: Rc<RefCell<Entry>>,
        pub material_selector: Rc<RefCell<ComboBoxText>>,
        pub color_button: Rc<RefCell<gtk::ColorButton>>,
    }

    pub struct FlatPlaneConfig {
        pub id: Rc<RefCell<u32>>,
        pub pos_x_entry: Rc<RefCell<Entry>>,
        pub pos_y_entry: Rc<RefCell<Entry>>,
        pub pos_z_entry: Rc<RefCell<Entry>>,
        pub radius_entry: Rc<RefCell<Entry>>,
        pub material_selector: Rc<RefCell<ComboBoxText>>,
        pub color_button: Rc<RefCell<gtk::ColorButton>>,
    }

    pub mod interface;
    pub use interface::*;

    pub mod update;
    pub use update::*;

    pub mod validate;
    pub use validate::*;

    pub mod components {
        pub use super::*;

        pub mod entries;
        pub use entries::*;

        pub mod objects;
        pub use objects::*;

        pub mod buttons;
        pub use buttons::*;

        pub mod scales;
        pub use scales::*;

        pub mod about;
        pub use about::*;
    }
}

pub mod raytracer {

    pub mod camera;
    pub use camera::*;
    pub mod ray;
    pub use ray::*;
    pub mod scene;
    pub use scene::*;
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
    use crate::textures::Texture;
    use crate::type_aliases::{Normal, Point};
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
        fn intersection(&self, ray: &Ray) -> Option<Intersection>;
        fn texture(&self) -> Texture;
    }

    pub type Objects = Vec<Arc<dyn Object>>;

    pub type Distance = f64;

    pub struct Intersection {
        pub hit_point: Point,
        pub normal: Normal,
        pub distance: Distance,
        pub texture: Texture,
    }

    impl Intersection {
        pub fn new(hit_point: Point, normal: Normal, distance: Distance, texture: Texture) -> Self {
            Self {
                hit_point,
                normal,
                distance,
                texture,
            }
        }
    }
}

pub mod textures {
    use crate::type_aliases::Color;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Texture {
        Light(Color),
        Diffusive(Color),
        Reflective,
    }
}
