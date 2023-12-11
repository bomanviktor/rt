use crate::type_aliases::Directions;
use crate::{config::rays::*, textures::Texture, type_aliases::Color};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
    pub collisions: Vec<Color>,
    pub hit_light_source: bool,
    pub intersection_dist: f64,
    pub depth: u8,
}

impl Ray {
    pub fn new(origin: Point, direction: Point, depth: u8) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            collisions: Vec::new(),
            hit_light_source: false,
            intersection_dist: f64::MAX,
            depth,
        }
    }

    /// Find the closest intersection, create a new ray based on the surface, and recursively call
    /// `trace`
    pub fn trace(&mut self, scene: &Scene) {
        if self.reached_max_depth() {
            return;
        }

        // Process the closest intersection
        if let Some(intersection) = self.closest_intersection(scene) {
            let origin = intersection.hit_point;
            let normal = intersection.normal;

            // Reflect based on object texture
            match intersection.texture {
                Texture::Diffusive(color) => {
                    self.collisions.push(color);
                    let direction = self.diffusive_direction(normal);
                    if direction.near_zero() {
                        self.reflect(origin, normal, scene);
                    } else {
                        self.reflect(origin, direction, scene);
                    }
                }
                Texture::Reflective => {
                    let direction = self.perfect_reflection(normal);
                    self.reflect(origin, direction, scene);
                }

                Texture::Light(color) => {
                    self.collisions.push(color);
                    self.hit_light_source = true;
                }
            }
        }
    }

    fn closest_intersection(&mut self, scene: &Scene) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;
        for object in &scene.objects {
            if let Some(intersection) = object.intersection(self) {
                if intersection.distance < self.intersection_dist {
                    self.intersection_dist = intersection.distance;
                    closest_intersection = Some(intersection);
                }
            }
        }
        closest_intersection
    }

    /// ### diffuse_reflection_direction
    ///
    /// Generate a random direction for diffuse reflection on a hemisphere given a surface normal
    fn diffusive_direction(&self, normal: Normal) -> Direction {
        let mut rng = rand::thread_rng();

        // Create a local coordinate system around the normal
        let incident_ray = normal.normalize();
        let tangent_a = if incident_ray.x.abs() > 0.90 {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(-1.0, 0.0, 0.0)
        };
        let tangent_v = incident_ray.cross(&tangent_a).normalize();
        let tangent_u = incident_ray.cross(&tangent_v);

        // Generate random points on a hemisphere
        let scaled_r1: f64 = rng.gen_range(-1.0..1.0);
        let scaled_r2: f64 = rng.gen_range(-1.0..1.0);
        let sin_theta = (1.0 - scaled_r2 * scaled_r2).sqrt();
        let phi = 2.0 * std::f64::consts::PI * scaled_r1;
        let local_x = phi.cos() * sin_theta;
        let local_y = phi.sin() * sin_theta;
        let local_z = scaled_r2.sqrt();

        // Convert to world coordinates
        tangent_u * local_x + tangent_v * local_y + incident_ray * local_z
    }

    pub fn reflect(&mut self, origin: Point, direction: Direction, scene: &Scene) {
        let mut secondary_ray = Ray::new(origin, direction, self.depth + 1);

        secondary_ray.trace(scene);

        self.collisions.extend(secondary_ray.collisions);

        if secondary_ray.hit_light_source {
            self.hit_light_source = true;
        }
    }

    fn perfect_reflection(&self, normal: Normal) -> Direction {
        self.direction - 2.0 * self.direction.dot(&normal) * normal
    }

    fn reached_max_depth(&self) -> bool {
        self.depth >= MAX_DEPTH
    }
}
