use crate::color::Color;
use crate::config::Point;
use crate::objects::{Intersection, Object};
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Cube {
    pub center: Point,
    pub size: f64,
    pub color: Color,
}

impl Cube {
    pub fn new(center: Point, size: f64, color: Color) -> Self {
        Self {
            center,
            size,
            color,
        }
    }

    fn check_plane_intersection(
        &self,
        ray: &Ray,
        plane_center: Point,
        normal: Vector3<f64>,
    ) -> Intersection {
        let denom = normal.dot(&ray.direction);
        if denom.abs() > 1e-6 {
            let v = plane_center - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some((ray.origin + ray.direction * distance, distance));
            }
        }
        None
    }
}

impl Object for Cube {
    fn intersection(&self, ray: &Ray) -> Intersection {
        let mut closest_intersection: Intersection = None;
        let half_size = self.size / 2.0;
        let (x, y, z) = (0, 1, 2);

        for axis in [x, y, z] {
            for sign in [-1.0, 1.0] {
                let mut normal = Vector3::zeros();
                normal[axis] = sign;
                let face_center = self.center + half_size * normal;

                if let Some((point, dist)) = self.check_plane_intersection(ray, face_center, normal)
                {
                    // Check if point is within cube bounds
                    let local_point = point - self.center;
                    if local_point.iter().all(|&coord| coord.abs() <= half_size)
                        && (closest_intersection.is_none()
                            || closest_intersection.unwrap().1 > dist)
                    {
                        closest_intersection = Some((point, dist));
                    }
                }
            }
        }
        closest_intersection
    }

    fn normal_at(&self, point: Point) -> Vector3<f64> {
        // Assuming the cube is axis-aligned, the normal can be derived from the point's closest axis
        let local_point = point - self.center;
        let mut max_axis = 0;
        let mut max_value = 0.0;
        let (x, y, z) = (0, 1, 2);

        for axis in [x, y, z] {
            if local_point[axis].abs() > max_value {
                max_axis = axis;
                max_value = local_point[axis].abs();
            }
        }

        let mut normal = Vector3::zeros();
        normal[max_axis] = local_point[max_axis].signum();
        normal
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
