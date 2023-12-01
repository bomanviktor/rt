use crate::config::Point;
use crate::objects::{Intersection, Object};
use crate::raytracer::Ray;
use nalgebra::Vector3;

use super::Texture;

#[derive(Debug, Clone)]
pub struct FlatPlane {
    pub center: Point,
    pub radius: f64,
    pub color: Vector3<f64>,
    pub texture: Texture,
}

impl FlatPlane {
    pub fn new(center: Point, radius: f64, color: Vector3<f64>, texture: Texture) -> Self {
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
        let plane_normal = Vector3::new(0.0, -1.0, 0.0);
        let denom = ray.direction.dot(&plane_normal);

        if denom.abs() > 1e-6 {
            let dist = (self.center - ray.origin).dot(&plane_normal) / denom;
            if (f64::EPSILON..ray.intersection_dist).contains(&dist) {
                let hit_point = ray.origin + ray.direction * dist;
                if (hit_point - self.center).norm() <= self.radius {
                    return Some((hit_point, dist));
                }
            }
        }

        None
    }

    fn normal_at(&self, ray: &Ray, _point: Point) -> Vector3<f64> {
        if ray.origin.y <= self.center.y {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        }
    }
    fn color(&self) -> Vector3<f64> {
        self.color
    }
    fn texture(&self) -> Texture {
        self.texture
    }
    fn center(&self) -> Point {
        self.center
    }
    fn is_light(&self) -> bool {
        self.texture == Texture::Light
    }
}
