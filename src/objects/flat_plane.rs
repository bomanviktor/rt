use crate::color::Color;
use crate::config::Point;
use crate::objects::{Intersection, Object};
use crate::raytracer::Ray;
use nalgebra::Vector3;

use super::Texture;

#[derive(Debug, Clone)]
pub struct FlatPlane {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub texture: Texture,
}

impl FlatPlane {
    pub fn new(center: Point, radius: f64, color: Color, texture: Texture) -> Self {
        Self {
            center,
            radius,
            color,
            texture,
        }
    }
}

impl Object for FlatPlane {
    fn intersection(&self, ray: &Ray) -> Intersection {
        const EPSILON: f64 = 1e-10; // A small value to handle floating point imprecision
        let plane_normal = self.normal_at(ray, Point::default()); // Assuming a horizontal plane

        let denominator = ray.direction.dot(&plane_normal);
        if denominator.abs() > EPSILON {
            // Ensuring we are not dividing by a very small number
            let t = (self.center - ray.origin).dot(&plane_normal) / denominator;
            if t > EPSILON {
                let hit_point = ray.origin + ray.direction * t;
                if (hit_point - self.center).norm() <= self.radius {
                    return Some((hit_point, t));
                }
            }
        }
        None
    }

    fn normal_at(&self, ray: &Ray, _point: Vector3<f64>) -> Vector3<f64> {
        if ray.origin.y > self.center.y {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        }
    }
    fn color(&self) -> Color {
        self.color
    }
    fn texture(&self) -> Texture {
        self.texture
    }
}
