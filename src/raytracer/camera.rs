use crate::color::Color;
use crate::config::{Pixels, Point};
use crate::raytracer::{Ray, Resolution, Scene};
use nalgebra::Vector3;
use rayon::prelude::*;
use std::io::Write;
use std::sync::Arc;

const DEFAULT_CAMERA_POSITION: Point = Point::new(1.0, 0.5, 0.0);
const DEFAULT_SAMPLE_SIZE: u16 = 1;
const DEFAULT_FOCAL_LENGTH: f64 = 0.5;
const DEFAULT_SENSOR_WIDTH: f64 = 1.0;

const DEFAULT_UP_DIRECTION: Point = Point::new(0.0, 1.0, 0.0);

const DEFAULT_RESOLUTION: Resolution = (1600, 900);

#[derive(Debug)]
pub struct Camera {
    pub sample_size: u16,
    pub position: Vector3<f64>,
    pub look_at: Vector3<f64>,
    pub up_direction: Vector3<f64>,
    pub fov: f64,
    pub resolution: Resolution,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub sensor_width: f64,
    pub rays: Vec<Vec<Ray>>,
    pub pixels: Pixels,
}

impl Camera {
    pub fn send_rays(&mut self, scene: Arc<Scene>) {
        // Using Rayon's parallel iterator for processing each row in parallel
        self.pixels = self
            .rays
            .par_iter_mut()
            .map(|row| {
                let mut pixel_row = Vec::with_capacity(row.len());
                for ray in row.iter_mut() {
                    ray.trace(&scene, 0);

                    // Take out !ray.hit_light_source to render based on the normal vector.
                    // Leave it in to render based on ray-tracing.
                    if !ray.hit_light_source || ray.collisions.is_empty() {
                        pixel_row.push(Color::default());
                    } else {
                        pixel_row.push(ray.average_color())
                    }
                    // Clear the collisions for the next frame
                    ray.collisions.clear();
                }
                pixel_row
            })
            .collect();
    }

    pub fn write_to_ppm(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", self.pixels[0].len(), self.pixels.len()).unwrap();
        writeln!(file, "255").unwrap();
        for row in &self.pixels {
            for pixel in row {
                writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
            }
        }
    }
}

#[derive(Default)]
pub struct CameraBuilder {
    pub sample_size: Option<u16>,
    pub position: Option<Vector3<f64>>,
    pub look_at: Option<Vector3<f64>>,
    pub up_direction: Option<Vector3<f64>>,
    pub resolution: Option<Resolution>,
    pub focal_length: Option<f64>,
    pub sensor_width: Option<f64>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            sample_size: None,
            position: None,
            look_at: None,
            up_direction: None,
            resolution: None,
            focal_length: None,
            sensor_width: None,
        }
    }

    pub fn sample_size(&mut self, sample_size: u16) -> &mut Self {
        self.sample_size = Some(sample_size);
        self
    }

    pub fn position_by_coordinates(&mut self, position: Point) -> &mut Self {
        self.position = Some(position);
        self
    }

    pub fn position_by_degrees(
        &mut self,
        _horizontal_degrees: f64,
        _vertical_degrees: f64,
    ) -> &mut Self {
        self.look_at = None; // This is to trigger the default option in the builder
        self
    }

    pub fn look_at(&mut self, coordinate: Point) -> &mut Self {
        self.look_at = Some(coordinate);
        self
    }
    pub fn up_direction_by_coordinates(&mut self, up_direction: Point) -> &mut Self {
        self.up_direction = Some(up_direction);
        self
    }

    pub fn up_direction_by_rotation(&mut self, _rotation: f64) -> &mut Self {
        self
    }

    pub fn resolution(&mut self, resolution: Resolution) -> &mut Self {
        self.resolution = Some(resolution);
        self
    }

    pub fn focal_length(&mut self, focal_length: f64) -> &mut Self {
        self.focal_length = Some(focal_length);
        self
    }

    pub fn sensor_width(&mut self, sensor_width: f64) -> &mut Self {
        self.sensor_width = Some(sensor_width);
        self
    }

    fn ray_direction(&self, pixel_x: u32, pixel_y: u32) -> Vector3<f64> {
        // Calculate the camera basis vectors
        let view_direction = (self.position.unwrap() - self.look_at.unwrap()).normalize();
        let right_vector = self
            .up_direction
            .unwrap()
            .cross(&view_direction)
            .normalize();
        let up_vector = view_direction.cross(&right_vector);
        let (width, height) = self.resolution.unwrap();

        // Convert pixel coordinates to normalized world coordinates
        let normalized_x = (pixel_x as f64) / (width as f64) - 0.5;
        let normalized_y = (pixel_y as f64) / (height as f64) - 0.5;

        let aspect_ratio = width as f64 / height as f64;
        // Compute the ray direction
        right_vector * (normalized_x * aspect_ratio) + up_vector * normalized_y
            - view_direction * self.focal_length.unwrap()
    }

    pub fn map_ray_directions(&self) -> Vec<Vec<Ray>> {
        let mut rays = Vec::new();
        let (width, height) = self.resolution.unwrap();

        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let direction = self.ray_direction(x, y);
                row.push(Ray {
                    origin: self.position.unwrap(),
                    direction,
                    collisions: vec![],
                    hit_light_source: false,
                });
            }
            rays.push(row);
        }

        rays
    }

    pub fn build(&self) -> Camera {
        let fov = 2.0
            * ((self.sensor_width.unwrap_or(DEFAULT_SENSOR_WIDTH)
                / (2.0 * self.focal_length.unwrap_or(DEFAULT_FOCAL_LENGTH)))
            .atan());
        let (width, height) = self.resolution.unwrap_or(DEFAULT_RESOLUTION);
        Camera {
            sample_size: self.sample_size.unwrap_or(DEFAULT_SAMPLE_SIZE),
            position: self.position.unwrap_or(DEFAULT_CAMERA_POSITION),
            look_at: self.look_at.unwrap_or_default(), // 0,0,0 is the default
            up_direction: self.up_direction.unwrap_or(DEFAULT_UP_DIRECTION),
            fov,
            resolution: self.resolution.unwrap_or(DEFAULT_RESOLUTION),
            aspect_ratio: width as f64 / height as f64,
            focal_length: self.focal_length.unwrap_or(DEFAULT_FOCAL_LENGTH),
            sensor_width: self.sensor_width.unwrap_or(DEFAULT_SENSOR_WIDTH),
            rays: self.map_ray_directions(),
            pixels: Vec::new(),
        }
    }
}
