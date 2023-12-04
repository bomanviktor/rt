use crate::objects::{Intersection, Object};
use crate::raytracer::Ray;
use crate::type_aliases::{Normal, Point};
use nalgebra::Vector3;

use super::Texture;

#[derive(Debug, Clone)]
pub struct FlatPlane {
    pub center: Point,
    pub radius: f64,
    pub texture: Texture,
}

impl FlatPlane {
    pub fn new(center: Point, radius: f64, texture: Texture) -> Self {
        Self {
            center,
            radius,
            texture,
        }
    }
    fn normal(&self, ray: &Ray) -> Normal {
        if ray.origin.y <= self.center.y {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        }
    }
}

impl Object for FlatPlane {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let normal = self.normal(ray);
        let denom = ray.direction.dot(&normal);
        if denom.abs() <= 1e-6 {
            return None;
        }

        let dist = (self.center - ray.origin).dot(&normal) / denom;
        if !(denom.abs()..=ray.intersection_dist).contains(&dist) {
            return None;
        }

        let hit_point = ray.origin + ray.direction * dist;

        if (hit_point - self.center).norm() <= self.radius {
            return Some(Intersection::new(
                hit_point * 1.0001,
                normal,
                dist,
                self.texture(),
            ));
        }

        None
    }

    fn texture(&self) -> Texture {
        self.texture
    }
    fn center(&self) -> Point {
        self.center
    }
    fn is_light(&self) -> bool {
        matches!(self.texture, Texture::Light(_))
    }
}
