use crate::color::Color;
use crate::config::Point;
use crate::objects::{discriminant, FlatPlane, Intersection, Object};
use crate::raytracer::Ray;
use nalgebra::Vector3;

use super::Texture;

#[derive(Debug)]
pub struct Cylinder {
    pub center: Point,
    pub radius: f64,
    pub height: f64,
    pub bottom: FlatPlane,
    pub top: FlatPlane,
    pub color: Color,
    pub texture: Texture,
}

impl Cylinder {
    pub fn new(center: Point, radius: f64, height: f64, color: Color, texture: Texture) -> Self {
        let bottom = FlatPlane::new(center, radius, color, texture);
        let top = FlatPlane::new(
            Vector3::new(center.x, center.y + height, center.z),
            radius,
            color,
            texture,
        );
        Self {
            center,
            radius,
            height,
            bottom,
            top,
            color,
            texture,
        }
    }
}

impl Object for Cylinder {
    fn intersection(&self, ray: &Ray) -> Intersection {
        let bottom = self.bottom.center;
        let _top = self.top.center;
        let axis = Vector3::new(0.0, 1.0, 0.0); // Cylinder aligned along Y-axis
        let mut valid_intersections = Vec::new();

        // Check intersection with cylindrical surface
        let vec_to_ray = ray.origin - bottom;
        let projection = axis * vec_to_ray.dot(&axis);
        let effective_origin = vec_to_ray - projection;
        let effective_direction = ray.direction - axis * ray.direction.dot(&axis);

        let a = effective_direction.dot(&effective_direction);
        let b = 2.0 * effective_origin.dot(&effective_direction);
        let c = effective_origin.dot(&effective_origin) - self.radius.powi(2);

        if let Some(discriminant) = discriminant(a, b, c) {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            for &t in &[t1, t2] {
                if t > 0.0 {
                    let point = ray.origin + ray.direction * t;
                    let height = (point - bottom).dot(&axis);
                    if height >= 0.0
                        && height <= self.height
                        && (ray.closest_intersection_distance < 0.0
                            || t < ray.closest_intersection_distance)
                    {
                        valid_intersections.push((point, t));
                    }
                }
            }
        }

        // Check intersections with both caps
        if let Some(bottom_intersection) = self.bottom.intersection(ray) {
            valid_intersections.push(bottom_intersection);
        }
        if let Some(top_intersection) = self.top.intersection(ray) {
            valid_intersections.push(top_intersection);
        }

        // Find the closest valid intersection
        valid_intersections
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    fn normal_at(&self, _ray: &Ray, point: Point) -> Vector3<f64> {
        // Determine if the point is on the top or bottom cap
        if (point - self.top.center).norm() <= self.radius {
            Vector3::new(0.0, -1.0, 0.0) // Normal for the top cap
        } else if (point - self.bottom.center).norm() <= self.radius {
            Vector3::new(0.0, 1.0, 0.0) // Normal for the bottom cap
        } else {
            // Normal for the cylindrical surface
            let axis = Vector3::new(0.0, -1.0, 0.0);
            let projection = axis * (point - self.center).dot(&axis);
            (point - self.center - projection).normalize()
        }
    }
    fn color(&self) -> Color {
        self.color
    }

    fn texture(&self) -> Texture {
        self.texture
    }
    fn position(&self) -> Point {
        self.center
    }
}
