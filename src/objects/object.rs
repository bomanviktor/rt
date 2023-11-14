use crate::color::Color;
use crate::config::Point;
use crate::raytracer::Ray;

pub trait Object {
    fn can_hit(&self, ray: &Ray) -> Option<f64>;

    fn calculate_distance(&self, ray_x: f64, ray_y: f64) -> f64;
    fn point_of_contact(&self, ray: &Ray) -> (Point, Color);
    fn is_light_source(&self) -> bool;
}
