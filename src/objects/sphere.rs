use crate::objects::{Intersection, Object};
use crate::raytracer::Ray;
use crate::type_aliases::{Normal, Point};

use super::Texture;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub texture: Texture,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, texture: Texture) -> Self {
        Self {
            center,
            radius,
            texture,
        }
    }
    fn normal(&self, point: Point) -> Normal {
        (point - self.center).normalize()
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let origin_to_center = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * origin_to_center.dot(&ray.direction);
        let c = origin_to_center.dot(&origin_to_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let dist_1 = (-b - sqrt_discriminant) / (2.0 * a);
        let dist_2 = (-b + sqrt_discriminant) / (2.0 * a);

        let dist = if (0.0..dist_2).contains(&dist_1) {
            dist_1 // dist 1 is closer
        } else {
            dist_2 // dist 2 is closer
        };

        if (0.0..ray.intersection_dist).contains(&dist) {
            let hit_point = ray.origin + dist * ray.direction;
            let normal = self.normal(hit_point);
            return Some(Intersection::new(hit_point, normal, dist, self.texture()));
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
