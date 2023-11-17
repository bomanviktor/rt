use crate::color::Color;
use crate::config::Point;
use crate::objects::Object;
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<(Vector3<f64>, f64)> {
        let ray_to_sphere_center = ray.origin - self.center;
        let direction_dot_product = ray.direction.dot(&ray.direction);
        let oc_dot_direction = 2.0 * ray_to_sphere_center.dot(&ray.direction);
        let radius_squared = self.radius * self.radius;
        let distance_from_sphere_surface =
            ray_to_sphere_center.dot(&ray_to_sphere_center) - radius_squared;
        let discriminant = oc_dot_direction * oc_dot_direction
            - 4.0 * direction_dot_product * distance_from_sphere_surface;

        if discriminant > 0.0 {
            let intersection_distance =
                (-oc_dot_direction - discriminant.sqrt()) / (2.0 * direction_dot_product);
            if intersection_distance > 0.0 {
                return Some((
                    ray.origin + ray.direction * intersection_distance,
                    intersection_distance,
                )); // Return intersection point and distance
            }
        }
        None
    }

    fn normal_at(&self, point: Vector3<f64>) -> Vector3<f64> {
        (point - self.center).normalize()
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
