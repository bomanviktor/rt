use crate::color::Color;
use crate::config::Point;
use crate::objects::{discriminant, FlatPlane, Intersection, Object};
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Cylinder {
    pub center: Point,
    pub radius: f64,
    pub height: f64,
    pub bottom: FlatPlane,
    pub top: FlatPlane,
    pub color: Color,
}

impl Cylinder {
    pub fn new(center: Point, radius: f64, height: f64, color: Color) -> Self {
        let bottom = FlatPlane::new(center, radius, color.clone());
        let top = FlatPlane::new(
            Vector3::new(center.x, center.y + height, center.z),
            radius,
            color.clone(),
        );
        Self {
            center,
            radius,
            height,
            bottom,
            top,
            color,
        }
    }
}

impl Object for Cylinder {
    fn intersection(&self, ray: &Ray) -> Intersection {
        let bottom = self.bottom.center;
        let top = self.top.center;
        let axis = (top - bottom).normalize();

        // Check intersection with cylindrical surface
        let vec_to_ray = ray.origin - bottom;
        let projection = axis * vec_to_ray.dot(&axis);
        let effective_origin = vec_to_ray - projection;
        let effective_direction = ray.direction - axis * ray.direction.dot(&axis);

        let a = effective_direction.dot(&effective_direction);
        let b = 2.0 * effective_origin.dot(&effective_direction);
        let c = effective_origin.dot(&effective_origin) - self.radius * self.radius;

        let mut valid_intersections = Vec::new();

        if let Some(discriminant) = discriminant(a, b, c) {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            for &t in &[t1, t2] {
                let point = ray.origin + ray.direction * t;
                let height = (point - bottom).dot(&axis);
                if height >= 0.0 && height <= self.height {
                    valid_intersections.push((point, t));
                }
            }
        }

        // Check intersections with both caps
        if let Some(bottom) = self.bottom.intersection(ray) {
            valid_intersections.push(bottom);
        }
        if let Some(top) = self.top.intersection(ray) {
            valid_intersections.push(top);
        }

        // Find the closest valid intersection
        valid_intersections
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    fn normal_at(&self, point: Point) -> Vector3<f64> {
        // Determine if the point is on the top or bottom cap
        if (point - self.top.center).norm() <= self.radius {
            Vector3::new(0.0, 1.0, 0.0) // Normal for the top cap
        } else if (point - self.bottom.center).norm() <= self.radius {
            Vector3::new(0.0, -1.0, 0.0) // Normal for the bottom cap
        } else {
            // Normal for the cylindrical surface
            let axis = Vector3::new(0.0, 1.0, 0.0);
            let projection = axis * (point - self.center).dot(&axis);
            (point - self.center - projection).normalize()
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}
