use crate::color::Color;
use crate::config::Point;
use crate::objects::Object;
use crate::raytracer::Ray;
use nalgebra::Vector3;

use super::Texture;

#[derive(Debug)]
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
    fn intersection(&self, ray: &Ray) -> Option<(Vector3<f64>, f64)> {
        let axis = if self.center.y > ray.origin.y {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        };
        let t = (self.center - ray.origin).dot(&axis) / ray.direction.dot(&axis);
        if t > 0.0 {
            let hit_point = ray.origin + ray.direction * t;
            if (hit_point - self.center).norm() <= self.radius {
                return Some((hit_point, t));
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
