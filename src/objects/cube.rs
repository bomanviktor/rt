use crate::objects::{Intersection, Object, Texture};
use crate::raytracer::Ray;
use crate::type_aliases::{Normal, Point};

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

        let mut normal = Normal::default();
        normal[max] = local_point[max].signum(); // Set the correct component of the normal
        normal
    }
}

impl Object for Cube {
    /// Loops through the faces of the cube, finds the one with the closest intersection,
    /// and returns the `Intersection`
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_distance = f64::MAX;
        let mut hit_point = Point::default();

        let half_size = self.size / 2.0;
        let (x, y, z) = (0, 1, 2);

        // Check intersections with each face of the cube
        for axis in [x, y, z] {
            for sign in [1.0, -1.0] {
                let mut normal = Normal::default();
                normal[axis] = sign;
                let face_center = self.center + half_size * normal;

                // Catch degenerate reflections
                let denom = normal.dot(&ray.direction);
                if denom.abs() <= 1e-6 {
                    continue;
                }
                let face_center_to_origin = face_center - ray.origin;
                let distance = face_center_to_origin.dot(&normal) / denom;

                // Also catch degenerate reflections
                if !(1e-6..ray.intersection_dist).contains(&distance) {
                    continue;
                }

                let point = ray.origin + distance * ray.direction;
                let local_point = point - self.center;

                // Check if point is within cube bounds and that the distance
                // is shorter than the previously closest distance
                let small_offset = 1e-7;
                if local_point
                    .iter()
                    .all(|&coord| coord.abs() <= half_size + small_offset)
                    && distance < closest_distance
                {
                    // Add a small offset depending on texture

                    // Update closest intersection
                    closest_distance = distance;
                    hit_point = point;
                }
            }
        }

        if closest_distance < ray.intersection_dist {
            Some(Intersection::new(
                hit_point,
                self.normal(hit_point),
                closest_distance,
                self.texture(),
            ))
        } else {
            None // No intersection was found or closest intersection was too far away.
        }
    }

    fn texture(&self) -> Texture {
        self.texture
    }
}
