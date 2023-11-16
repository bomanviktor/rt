use crate::config::Point;
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Camera {
    pub sample_size: u64,
    pub position: Vector3<f64>,
    pub look_at: Vector3<f64>,
    pub up_direction: Vector3<f64>,
    pub fov: f64,
    pub resolution: (u32, u32),
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub sensor_width: f64,
    pub rays: Vec<Vec<Ray>>,
}

// TODO: Remove dead code macro
#[allow(dead_code)]
impl Camera {}

#[derive(Default)]
pub struct CameraBuilder {
    pub sample_size: Option<u64>,
    pub position: Option<Vector3<f64>>,
    pub look_at: Option<Vector3<f64>>,
    pub up_direction: Option<Vector3<f64>>,
    pub resolution: Option<(u32, u32)>,
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

    pub fn sample_size(&mut self, sample_size: u64) -> &mut Self {
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

    pub fn resolution(&mut self, resolution: (u32, u32)) -> &mut Self {
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
                });
            }
            rays.push(row);
        }

        rays
    }

    pub fn build(&self) -> Camera {
        let fov = 2.0 * ((self.sensor_width.unwrap() / (2.0 * self.focal_length.unwrap())).atan());
        let (width, height) = self.resolution.unwrap();
        Camera {
            sample_size: self.sample_size.unwrap(),
            position: self.position.unwrap(),
            look_at: self.look_at.unwrap(),
            up_direction: self.up_direction.unwrap(),
            fov,
            resolution: self.resolution.unwrap(),
            aspect_ratio: width as f64 / height as f64,
            focal_length: self.focal_length.unwrap(),
            sensor_width: self.sensor_width.unwrap(),
            rays: self.map_ray_directions(),
        }
    }
}
