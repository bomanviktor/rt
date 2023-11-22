use crate::color::Color;
use crate::config::Point;
use crate::objects::{discriminant, Object};
use crate::raytracer::Ray;
use nalgebra::Vector3;

use super::Texture;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub texture: Texture,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, color: Color, texture: Texture) -> Self {
        Self {
            center,
            radius,
            color,
            texture,
        }
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<(Vector3<f64>, f64)> {
        let ray_to_sphere_center = ray.origin - self.center;
        let oc_dot_direction = 2.0 * ray_to_sphere_center.dot(&ray.direction);
        let direction_dot_product = ray.direction.dot(&ray.direction);
        let distance_from_sphere_surface =
            ray_to_sphere_center.dot(&ray_to_sphere_center) - self.radius.powi(2);

        let a = direction_dot_product;
        let b = oc_dot_direction;
        let c = distance_from_sphere_surface;

        if let Some(discriminant) = discriminant(a, b, c) {
            let intersection_distance =
                (-oc_dot_direction - discriminant.sqrt()) / (2.0 * direction_dot_product);
            if intersection_distance > 0.0 {
                let intersection_point = ray.origin + ray.direction * intersection_distance;
                return Some((intersection_point, intersection_distance));
            }
        }
        None
    }

    fn normal_at(&self, point: Vector3<f64>) -> Vector3<f64> {
        (point - self.center).normalize()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn texture(&self) -> Texture {
        self.texture
    }
}
