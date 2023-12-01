use crate::config::Point;
use crate::objects::{Intersection, Object, Texture};
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Cube {
    pub center: Point,
    pub size: f64,
    pub color: Vector3<f64>,
    pub texture: Texture,
}

impl Cube {
    pub fn new(center: Point, size: f64, color: Vector3<f64>, texture: Texture) -> Self {
        Self {
            center,
            size,
            color,
            texture,
        }
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

                // Check if point is within cube bounds
                if local_point.iter().all(|&coord| coord.abs() <= half_size)
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
