use crate::color::Color;
use crate::config::Point;
use crate::objects::{discriminant, Intersection, Object};
use crate::raytracer::Ray;
use nalgebra::Vector3;

use super::Texture;

#[derive(Debug, Clone)]
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
    fn intersection(&self, ray: &Ray) -> Intersection {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            let t = if t1 > 0.0 && (t1 < t2 || t2 < 0.0) { t1 } else if t2 > 0.0 { t2 } else { return None; };

            if t > 0.0 && (ray.closest_intersection_distance < 0.0 || t < ray.closest_intersection_distance) {
                let point = ray.origin + t * ray.direction;
                return Some((point, t));
            }
        }
        None
    }


    fn normal_at(&self, _ray: &Ray, point: Vector3<f64>) -> Vector3<f64> {
        (point - self.center).normalize()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn texture(&self) -> Texture {
        self.texture
    }
}
