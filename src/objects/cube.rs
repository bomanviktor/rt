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
        let local_point = point - self.center;
        let max = local_point
            .iter()
            .map(|v| v.abs())
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap_or(0);

        let mut normal = Normal::default();
        normal[max] = local_point[max].signum();
        normal
    }
}

impl Object for Cube {
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        // Calculate the half size of the cube to determine its bounds.
        let half_size = self.size / 2.0;
        // Determine the minimum bounds of the cube in 3D space.
        let min_bounds = Point::new(
            self.center.x - half_size,
            self.center.y - half_size,
            self.center.z - half_size,
        );
        // Determine the maximum bounds of the cube in 3D space.
        let max_bounds = Point::new(
            self.center.x + half_size,
            self.center.y + half_size,
            self.center.z + half_size,
        );

        // Closure to calculate intersections on a single axis.
        // It returns the nearest and farthest intersection points along the axis.
        let intersect = |min_bound, max_bound, ray_origin, ray_direction| {
            let inv_dir = 1.0 / ray_direction;
            let t1: f64 = (min_bound - ray_origin) * inv_dir;
            let t2 = (max_bound - ray_origin) * inv_dir;
            (t1.min(t2), t1.max(t2))
        };

        // Calculate intersections for each axis.
        let (t_min_x, t_max_x) =
            intersect(min_bounds.x, max_bounds.x, ray.origin.x, ray.direction.x);
        let (t_min_y, t_max_y) =
            intersect(min_bounds.y, max_bounds.y, ray.origin.y, ray.direction.y);
        let (t_min_z, t_max_z) =
            intersect(min_bounds.z, max_bounds.z, ray.origin.z, ray.direction.z);

        // Determine the overall nearest and farthest intersection points.
        let t_min = t_min_x.max(t_min_y).max(t_min_z);
        let t_max = t_max_x.min(t_max_y).min(t_max_z);

        // If there is no valid intersection, return None.
        if t_min > t_max || t_max < 0.0 {
            return None;
        }

        // Determine the distance to the intersection point.
        let distance = if t_min >= 0.0 { t_min } else { t_max };
        // Calculate the exact hit point on the cube's surface.
        let hit_point = ray.origin + distance * ray.direction * 1.00001;
        // Calculate the normal at the hit point.
        let normal = self.normal(hit_point);

        // Return the intersection data, including hit point, normal, distance, and texture.
        Some(Intersection::new(
            hit_point,
            normal,
            distance,
            self.texture(),
        ))
    }

    fn texture(&self) -> Texture {
        self.texture
    }
}
