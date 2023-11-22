use crate::color::Color;
use crate::config::Point;
use crate::objects::{Object, Texture};
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Cube {
    pub center_point: Point,
    pub size: f64,
    pub color: Color,
    pub texture: Texture,
}

impl Cube {
    pub fn new(center_point: Point, size: f64, color: Color, texture: Texture) -> Self {
        Self {
            center_point,
            size,
            color,
            texture,
        }
    }

    fn check_plane_intersection(
        &self,
        ray: &Ray,
        plane_center: Vector3<f64>,
        normal: Vector3<f64>,
    ) -> Option<(Vector3<f64>, f64)> {
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
    fn intersection(&self, ray: &Ray) -> Option<(Vector3<f64>, f64)> {
        let half_size = self.size / 2.0;
        let mut closest_intersection: Option<(Vector3<f64>, f64)> = None;

        for &axis in &[0, 1, 2] {
            for &sign in &[-1.0, 1.0] {
                let mut normal = Vector3::zeros();
                normal[axis] = sign;
                let face_center = self.center_point + half_size * normal;

                if let Some((point, dist)) = self.check_plane_intersection(ray, face_center, normal)
                {
                    // Check if point is within cube bounds
                    let local_point = point - self.center_point;
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

    fn normal_at(&self, point: Vector3<f64>) -> Vector3<f64> {
        // Assuming the cube is axis-aligned, the normal can be derived from the point's closest axis
        let local_point = point - self.center_point;
        let mut max_axis = 0;
        let mut max_value = local_point[0].abs();

        for i in 1..3 {
            if local_point[i].abs() > max_value {
                max_axis = i;
                max_value = local_point[i].abs();
            }
        }

        let mut normal = Vector3::zeros();
        normal[max_axis] = local_point[max_axis].signum();
        normal
    }

    fn color(&self) -> Color {
        self.color
    }
    fn texture(&self) -> Texture {
        self.texture
    }
}
