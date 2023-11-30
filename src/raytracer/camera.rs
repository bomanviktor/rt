use crate::color::Color;
use crate::config::{Pixels, Point};
use crate::raytracer::{Ray, Resolution, Scene};
use nalgebra::Vector3;
use rayon::prelude::*;
use std::io::Write;
use std::ops::RangeInclusive;
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
        // Calculate the number of rays per pixel
        let samples_per_pixel = self.sample_size as usize; // Assuming square sample size (e.g., 4 for 2x2)

        // Prepare a vector to store the results of ray tracing
        let mut ray_results = Vec::with_capacity(self.rays.len() * samples_per_pixel);

        // Trace all rays and store the results
        for row in &self.rays {
            for ray in row {
                let mut ray_clone = ray.clone();
                ray_clone.trace(&scene, 0);
                ray_results.push(ray_clone);
            }
        }

        // Process the results to calculate pixel colors
        self.pixels.clear(); // Clear existing pixel data
        for row_chunks in ray_results.chunks(self.resolution.0 as usize * samples_per_pixel) {
            let mut pixel_row = Vec::with_capacity(self.resolution.0 as usize);
            for rays_chunk in row_chunks.chunks(samples_per_pixel) {
                let mut color_sum = Color::new(0, 0, 0);
                for ray in rays_chunk {
                    let color = if ray.collisions.is_empty() {
                        Color::default()
                    } else {
                        ray.average_color()
                    };
                    color_sum.r = color_sum.r.saturating_add(color.r);
                    color_sum.g = color_sum.g.saturating_add(color.g);
                    color_sum.b = color_sum.b.saturating_add(color.b);
                }
                color_sum.r /= samples_per_pixel as u8;
                color_sum.g /= samples_per_pixel as u8;
                color_sum.b /= samples_per_pixel as u8;
                pixel_row.push(color_sum);
            }
            self.pixels.push(pixel_row);
        }
    }

    pub fn write_to_ppm(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", self.pixels[0].len(), self.pixels.len()).unwrap();
        writeln!(file, "255").unwrap();

        let pixel_data: Vec<String> = self
            .pixels
            .par_iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| {
                        let corrected = pixel.apply_gamma_correction(2.2);
                        format!("{} {} {}", corrected.r, corrected.g, corrected.b)
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect();

        // Write the prepared pixel data to the file
        for row in pixel_data {
            writeln!(file, "{}", row).unwrap();
        }
    }

    pub fn anti_aliasing(&mut self, weight: f64) {
        if weight < 1.0 {
            panic!("wtf dude");
        }
        let w = self.resolution.0 as usize;
        let h = self.resolution.1 as usize;

        let mut anti_aliased_pixels = Vec::new();
        for row in 0..h {
            let mut anti_aliased_pixel_row = Vec::new();
            for column in 0..w {
                let mut total_r = self.pixels[row][column].r as f64 * weight;
                let mut total_g = self.pixels[row][column].g as f64 * weight;
                let mut total_b = self.pixels[row][column].b as f64 * weight;

                // TODO: Improve this BS.
                // Right now it is just getting the ranges that we will use to get the average.
                let mut rows: RangeInclusive<usize> = row - 1..=row + 1; // Every row except for:
                if row == 0 {
                    rows = 0..=1; // First row,
                }
                if row == 1 {
                    rows = 0..=2; // Second row,
                }
                if row == h - 1 {
                    rows = h - 2..=h - 1; // Last row.
                }

                let mut cols: RangeInclusive<usize> = column - 1..=column + 1;
                if column == 0 {
                    cols = 0..=1;
                }
                if column == 1 {
                    cols = 0..=2;
                }
                if column == w - 1 {
                    cols = w - 2..=w - 1
                }

                // Add all the colors of the ranges together
                let mut total_pixels = weight;
                for y in rows.clone() {
                    for x in cols.clone() {
                        total_pixels += 1.0;
                        total_r += self.pixels[y][x].r as f64;
                        total_g += self.pixels[y][x].g as f64;
                        total_b += self.pixels[y][x].b as f64;
                    }
                }

                anti_aliased_pixel_row.push(Color::new(
                    (total_r / total_pixels) as u8,
                    (total_g / total_pixels) as u8,
                    (total_b / total_pixels) as u8,
                ))
            }
            anti_aliased_pixels.push(anti_aliased_pixel_row);
        }
        self.pixels = anti_aliased_pixels;
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

    fn ray_direction(
        &self,
        pixel_x: u32,
        pixel_y: u32,
        jitter_x: f64,
        jitter_y: f64,
    ) -> Vector3<f64> {
        // Calculate the camera basis vectors
        let view_direction = (self.position.unwrap() - self.look_at.unwrap()).normalize();
        let right_vector = self
            .up_direction
            .unwrap()
            .cross(&view_direction)
            .normalize();
        let up_vector = view_direction.cross(&right_vector);
        let (width, height) = self.resolution.unwrap_or(DEFAULT_RESOLUTION);

        // Convert pixel coordinates to normalized world coordinates
        let normalized_x = (pixel_x as f64 + jitter_x) / (width as f64) - 0.5;
        let normalized_y = (pixel_y as f64 + jitter_y) / (height as f64) - 0.5;

        let aspect_ratio = width as f64 / height as f64;
        // Compute the ray direction
        right_vector * (normalized_x * aspect_ratio) + up_vector * normalized_y
            - view_direction * self.focal_length.unwrap()
    }

    pub fn map_ray_directions(&self) -> Vec<Vec<Ray>> {
        let mut rays = Vec::new();
        let (width, height) = self.resolution.unwrap_or(DEFAULT_RESOLUTION);
        let sample_size_sqrt =
            (self.sample_size.unwrap_or(DEFAULT_SAMPLE_SIZE) as f64).sqrt() as u32;

        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                for _ in 0..sample_size_sqrt {
                    for _ in 0..sample_size_sqrt {
                        let jitter_x = (rand::random::<f64>() - 0.5) / sample_size_sqrt as f64;
                        let jitter_y = (rand::random::<f64>() - 0.5) / sample_size_sqrt as f64;
                        let direction = self.ray_direction(x, y, jitter_x, jitter_y);
                        row.push(Ray {
                            origin: self.position.unwrap(),
                            direction,
                            collisions: vec![],
                            hit_light_source: false,
                            closest_intersection_distance: -1.0,
                        });
                    }
                }
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
