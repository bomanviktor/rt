use crate::objects::{Intersection, Object, Texture};
use crate::raytracer::Ray;
use crate::type_aliases::{Normal, Point};
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Cube {
    pub center: Point,
    pub size: f64,
    pub texture: Texture,
}

impl Cube {
    pub fn new(center: Point, size: f64, texture: Texture) -> Self {
        Self {
            center,
            size,
            texture,
        }
    }

    fn normal(&self, point: Point) -> Normal {
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
}

impl Object for Cube {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_distance = f64::MAX;
        let mut hit_point = Point::default();

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

                if !(denom.abs()..=ray.intersection_dist).contains(&distance) {
                    continue;
                }

                let point = ray.origin + distance * ray.direction;
                let local_point = point - self.center;

                let float_offset = 1.00001;
                // Check if point is within cube bounds

                if local_point
                    .iter()
                    .all(|&coord| coord.abs() <= half_size * float_offset)
                    && distance < closest_distance
                {
                    // Update closest intersection
                    closest_distance = distance;
                    hit_point = point * float_offset;
                }
            }
        }

        if closest_distance < ray.intersection_dist {
            let normal = self.normal(hit_point);
            Some(Intersection::new(
                hit_point,
                normal,
                closest_distance,
                self.texture(),
            ))
        } else {
            None
        }
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
