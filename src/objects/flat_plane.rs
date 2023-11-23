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
        let axis = self.normal_at(ray, Point::default());
        let t = (self.center - ray.origin).dot(&axis) / ray.direction.dot(&axis);
        if t > 0.0 {
            let hit_point = ray.origin + ray.direction * t;
            if (hit_point - self.center).norm() <= self.radius {
                return Some((hit_point, t));
            }
        }
        None
    }

    fn normal_at(&self, ray: &Ray, _point: Vector3<f64>) -> Vector3<f64> {
        if ray.origin.y <= self.center.y {
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
