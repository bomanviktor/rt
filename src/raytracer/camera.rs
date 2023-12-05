use crate::config::camera::*;
use crate::type_aliases::Color;

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
    pub pixels: Pixels,
}

impl Camera {
    pub fn send_rays(&mut self, scene: Arc<Scene>) {
        let (width, height) = self.resolution;
        let total_pixels = (width * height) as usize;

        // Pre-allocate a vector with default Color values
        let mut colors = vec![Vector3::default(); total_pixels];

        // Parallelize the computation for each pixel
        colors
            .par_iter_mut()
            .enumerate()
            .for_each(|(pixel, pixel_color)| {
                let column = pixel as u32 % width;
                let row = pixel as u32 / width;
                let mut total_color = Color::black();

                for _sample in 0..self.sample_size {
                    let direction = self.ray_direction(column, row);
                    let mut ray = Ray::new(self.position, direction, 0);

                    ray.trace(&scene); // Recursive ray tracing with default 50 depth.

                    if ray.collisions.is_empty() {
                        let rgb = 255. * scene.brightness;
                        let background_color = Vector3::new(rgb, rgb, rgb);
                        total_color += background_color; // No collision, add void color.
                        continue;
                    }

                    if ray.hit_light_source {
                        total_color += ray.average_color(&scene);
                    } else {
                        total_color += ray.average_color(&scene) * scene.brightness
                    }
                }

                // Set the current pixel to the average color of the samples.
                *pixel_color = total_color / self.sample_size as f64;
            });

        // Update the camera's pixels
        self.pixels = colors;
    }

    pub fn write_to_ppm(&self, path: &str) {
        let (w, h) = self.resolution;
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(file, "P3").unwrap();
        writeln!(file, "{w} {h}").unwrap();
        writeln!(file, "255").unwrap();
        let pixel_data: Vec<String> = self
            .pixels
            .par_iter()
            .chunks(w as usize)
            .map(|row| {
                row.iter()
                    .map(|pixel| {
                        let corrected = pixel.correct_gamma(2.0);
                        format!("{} {} {}", corrected.r(), corrected.g(), corrected.b())
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

    fn ray_direction(&self, pixel_x: u32, pixel_y: u32) -> Vector3<f64> {
        // Calculate the camera basis vectors
        let view_direction = (self.position - self.look_at).normalize();
        let right_vector = self.up_direction.cross(&view_direction).normalize();
        let up_vector = view_direction.cross(&right_vector);
        let (width, height) = self.resolution;
        let mut rand = rand::thread_rng();

        // Convert pixel coordinates to normalized world coordinates
        let normalized_x = (pixel_x as f64 + rand.gen_range(0.0..1.0)) / (width as f64) - 0.5;
        let normalized_y = (pixel_y as f64 + rand.gen_range(0.0..1.0)) / (height as f64) - 0.5;

        // Compute the ray direction
        right_vector * (normalized_x * self.aspect_ratio) + up_vector * normalized_y
            - view_direction * self.focal_length
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

    pub fn resolution(&mut self, w: u32, h: u32) -> &mut Self {
        self.resolution = Some((w, h) as Resolution);
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
            pixels: Vec::new(),
        }
    }
}
