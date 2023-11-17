use crate::color::Color;
use crate::config::Point;
use crate::objects::Object;
use crate::raytracer::Ray;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct Cylinder {
    pub center: Point,
    pub radius: f64,
    pub height: f64,
    pub color: Color,
}

impl Cylinder {
    pub fn new(center: Point, radius: f64, height: f64, color: Color) -> Self {
        Self {
            center,
            radius,
            height,
            color,
        }
    }
}

impl Object for Cylinder {
    fn intersection(&self, ray: &Ray) -> Option<(Vector3<f64>, f64)> {
        let bottom = self.center;
        let top = Vector3::new(self.center.x, self.center.y + self.height, self.center.z);

        let axis = (top - bottom).normalize();

        let vec_to_ray = ray.origin - bottom;

        let projection = axis * vec_to_ray.dot(&axis);
        let effective_direction = ray.direction - axis * ray.direction.dot(&axis);

        let effective_origin = vec_to_ray - projection;

        let a = effective_direction.dot(&effective_direction);
        let b = 2.0 * effective_origin.dot(&effective_direction);
        let c = effective_origin.dot(&effective_origin) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        let mut valid_intersections = vec![];

        for &t in &[t1, t2] {
            let point = ray.origin + ray.direction * t;
            let height = (point - bottom).dot(&axis);
            if height > 0.0 && height < self.height {
                valid_intersections.push((point, t));
            }
        }

        valid_intersections
            .clone()
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut cap_intersections = Vec::new();

        let bottom_plane = self.center;
        let top_plane = Vector3::new(self.center.x, self.center.y + self.height, self.center.z);

        // Function to check intersection with a plane at `plane_center`
        let mut check_plane_intersection = |plane_center: Vector3<f64>| {
            let axis = Vector3::new(0.0, 1.0, 0.0); // Assuming cylinder's axis is aligned with y-axis
            let t = (plane_center - ray.origin).dot(&axis) / ray.direction.dot(&axis);
            if t > 0.0 {
                let hit_point = ray.origin + ray.direction * t;
                if (hit_point - plane_center).len() as f64 <= self.radius {
                    cap_intersections.push((hit_point, t));
                }
            }
        };

        // Check intersections with both caps
        check_plane_intersection(bottom_plane);
        check_plane_intersection(top_plane);

        // debug print if plane intersections are found
        if !cap_intersections.is_empty() {
            println!("Plane intersections: {:?}", cap_intersections);
        }

        // Combine all intersections and find the closest one
        valid_intersections.append(&mut cap_intersections);
        valid_intersections
            .into_iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    fn normal_at(&self, point: Vector3<f64>) -> Vector3<f64> {
        // Assuming the cylinder's axis is aligned with the y-axis
        let top_plane_center =
            Vector3::new(self.center.x, self.center.y + self.height, self.center.z);

        let _bottom_plane_center = self.center;

        // Check if the point is on the top or bottom cap
        if (point - top_plane_center).len() as f64 <= self.radius {
            Vector3::new(0.0, 1.0, 0.0) // Normal for the top cap
        } else {
            // Normal calculation for the cylindrical surface
            let axis = Vector3::new(0.0, 1.0, 0.0);
            let projection = axis * (point - self.center).dot(&axis);
            (point - self.center - projection).normalize()
        }
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
}
