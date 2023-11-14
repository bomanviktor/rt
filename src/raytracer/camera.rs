use crate::config::{Pixel, Point};
use crate::raytracer::{Ray, ViewPort};

#[derive(Debug)]
pub struct Camera {
    pub sample_size: u64,
    pub max_length: f64,
    pub position: Point,
    pub view_port: ViewPort,
    pub rays: Vec<Ray>,
}

// TODO: Remove dead code macro
#[allow(dead_code)]
impl Camera {
    pub fn new(sample_size: u64, max_length: f64, position: Point, view_port: ViewPort) -> Self {
        Self {
            sample_size,
            max_length,
            position,
            view_port,
            rays: Vec::new(),
        }
    }

    fn send_rays(&mut self) {
        let vp = &self.view_port;
        let top_y = vp.top_left().y as i64;
        let bottom_y = vp.bottom_left().y as i64;
        let left_x = vp.top_left().x as i64;
        let right_x = vp.top_right().x as i64;

        for _ in 0..self.sample_size {
            for y in top_y..=bottom_y {
                for x in left_x..=right_x {
                    self.rays.push(Ray::new(
                        self.position,
                        self.calc_direction(x, y),
                        Pixel::new(x, y),
                    ))
                }
            }

            self.calculate_rays()
        }
    }

    fn calculate_rays(&mut self) {
        for ray in &self.rays {
            println!("{:?}", ray);
            // Do calculation here
        }
        self.rays.clear()
    }

    fn calc_direction(&self, x: i64, y: i64) -> Point {
        let distance = self.view_port.distance;
        let x0 = self.position.x;
        let y0 = self.position.y;
        let x1 = x as f64;
        let y1 = y as f64;

        let x_diff = (x1 - x0) / distance;
        let y_diff = (y1 - y0) / distance;

        Point::new(
            x_diff * self.max_length,
            y_diff * self.max_length,
            self.max_length,
        )
    }
}
