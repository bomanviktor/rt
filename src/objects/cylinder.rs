use super::Texture;
use crate::objects::{discriminant, FlatPlane, Intersection, Object};
use crate::raytracer::Ray;
use crate::type_aliases::{Direction, Directions, Normal, Point};

#[derive(Debug)]
pub struct Cylinder {
    pub center: Point,
    pub radius: f64,
    pub height: f64,
    pub bottom: FlatPlane,
    pub top: FlatPlane,
    pub texture: Texture,
}

impl Cylinder {
    pub fn new(center: Point, radius: f64, height: f64, texture: Texture) -> Self {
        let bottom = FlatPlane::new(center, radius, texture);
        let top = FlatPlane::new(
            Point::new(center.x, center.y + height, center.z),
            radius,
            texture,
        );
        Self {
            center,
            radius,
            height,
            bottom,
            top,
            texture,
        }
    }

    fn normal(&self, point: Point) -> Normal {
        // Determine if the point is on the top or bottom cap
        if (point - self.top.center).norm() <= self.radius {
            Normal::up() // Normal for the top cap
        } else if (point - self.bottom.center).norm() <= self.radius {
            Normal::down() // Normal for the bottom cap
        } else {
            // Normal for the cylindrical surface
            let axis = Direction::down();
            let projection = axis * (point - self.center).dot(&axis);
            (point - self.center - projection).normalize()
        }
    }
}

impl Object for Cylinder {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        let bottom = self.bottom.center;
        let axis = Direction::up(); // Cylinder aligned along Y-axis
        let mut valid_intersections = Vec::new();

        // Check intersection with cylindrical surface
        let vec_to_ray = ray.origin - bottom;
        let projection = axis * vec_to_ray.dot(&axis);
        let effective_origin = vec_to_ray - projection;
        let effective_direction = ray.direction - axis * ray.direction.dot(&axis);

        let a = effective_direction.dot(&effective_direction);
        let b = 2.0 * effective_origin.dot(&effective_direction);
        let c = effective_origin.dot(&effective_origin) - self.radius.powi(2);

        // Check intersection on the cylindrical surface
        if let Some(discriminant) = discriminant(a, b, c) {
            let sqrt_discriminant = discriminant.sqrt();
            let dist_1 = (-b - sqrt_discriminant) / (2.0 * a);
            let dist_2 = (-b + sqrt_discriminant) / (2.0 * a);

            for dist in [dist_1, dist_2] {
                if dist <= 0.0 {
                    continue;
                }

                let point = ray.origin + ray.direction * dist;
                let height = (point - bottom).dot(&axis);

                if (0.0..=self.height).contains(&height) && dist < ray.intersection_dist {
                    // Add a small offset depending on texture
                    let offset = if matches!(self.texture, Texture::Reflective) {
                        1.0 + 1e-7
                    } else {
                        1.0
                    };

                    valid_intersections.push(Intersection::new(
                        point * offset,
                        self.normal(point),
                        dist,
                        self.texture,
                    ));
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

        // Find the closest valid intersection. If no intersection was found. Return None.
        valid_intersections
            .into_iter()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }

    fn texture(&self) -> Texture {
        self.texture
    }
}
