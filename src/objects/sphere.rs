use crate::objects::{Intersection, Object};
use crate::raytracer::Ray;
use crate::type_aliases::{Color, Normal, Point};

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

        if dist > 0.0 && dist < ray.intersection_dist {
            let point = ray.origin + dist * ray.direction;
            return Some((point, dist));
        }

        None
    }

    fn normal_at(&self, _ray: &Ray, point: Point) -> Normal {
        (point - self.center).normalize()
    }

    fn color(&self) -> Color {
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
