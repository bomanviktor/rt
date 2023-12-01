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
        let (x, y, z) = (0, 1, 2);
        let (pos, neg) = (1.0, -1.0);

        // Check intersections with each face of the cube
        for axis in [x, y, z] {
            for sign in [pos, neg] {
                let mut normal = Vector3::zeros();
                normal[axis] = sign;
                let face_center = self.center + half_size * normal;

                let denom = normal.dot(&ray.direction);
                if denom.abs() <= 1e-6 {
                    continue;
                }
                let face_center_to_origin = face_center - ray.origin;
                let distance = face_center_to_origin.dot(&normal) / denom;

                if !(denom.abs()..ray.intersection_dist).contains(&distance) {
                    continue;
                }

                let point = ray.origin + distance * ray.direction;
                let local_point = point - self.center;

                let float_offset = 1.00001;
                // Check if point is within cube bounds
                if local_point
                    .iter()
                    .all(|&coord| coord.abs() <= half_size * float_offset)
                    && (closest_intersection.is_none()
                        || distance < closest_intersection.unwrap().1)
                {
                    // Update closest intersection
                    closest_intersection = Some((point * float_offset, distance));
                }
            }
        }
        closest_intersection
    }

    fn normal_at(&self, _ray: &Ray, point: Point) -> Vector3<f64> {
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
