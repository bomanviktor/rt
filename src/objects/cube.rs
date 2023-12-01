use crate::color::Color;
use crate::config::Point;
use crate::objects::{Intersection, Object, Texture};
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Cube {
    pub center: Point,
    pub size: f64,
    pub color: Color,
    pub texture: Texture,
}

impl Cube {
    pub fn new(center: Point, size: f64, color: Color, texture: Texture) -> Self {
        Self {
            center,
            size,
            color,
            texture,
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

impl Object for Cube {
    fn intersection(&self, ray: &Ray) -> Intersection {
        let mut closest_intersection: Intersection = None;
        let half_size = self.size / 2.0;

        // Check intersections with each face of the cube
        for axis in 0..3 {
            for sign in [-1.0, 1.0].iter() {
                let mut normal = Vector3::zeros();
                normal[axis] = *sign;
                let face_center = self.center + half_size * normal;

                let denom = normal.dot(&ray.direction);
                if denom.abs() <= 1e-6 {
                    continue;
                }
                let v = face_center - ray.origin;
                let distance = v.dot(&normal) / denom;

                if distance < 0.0 || distance > ray.intersection_dist {
                    continue;
                }

                let point = ray.origin + distance * ray.direction;
                let local_point = point - self.center;

                let float_offset = 1.0001 * 1.0001;

                // Check if point is within cube bounds
                if local_point
                    .iter()
                    .all(|&coord| coord.abs() <= half_size * float_offset)
                    && (closest_intersection.is_none()
                        || closest_intersection.unwrap().1 > distance)
                {
                    // Update closest intersection
                    closest_intersection = Some((point, distance));
                }
            }
        }
        closest_intersection
    }

    fn normal_at(&self, _ray: &Ray, point: Point) -> Vector3<f64> {
        let _half_size = self.size / 2.0;
        let local_point = point - self.center; // Convert the point to the cube's local space

        // Determine which face the point is on by finding the largest component of the local point
        let max = local_point
            .iter()
            .map(|v| v.abs())
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        let mut normal = Vector3::zeros();
        normal[max] = local_point[max].signum(); // Set the correct component of the normal
        normal
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
