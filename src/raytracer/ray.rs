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
                    let direction = self.reflective_direction(normal, 0.9);
                    if direction.near_zero() {
                        self.diffusive(origin, normal, scene);
                    } else {
                        self.diffusive(origin, direction, scene);
                    }
                }
                Texture::Glossy(color) => {
                    self.collisions.push(color);
                    let direction = self.reflective_direction(normal, 0.6);
                    if direction.near_zero() {
                        self.diffusive(origin, normal, scene);
                    } else {
                        self.diffusive(origin, direction, scene);
                    }
                }

                Texture::Reflective => {
                    let direction = self.reflective_direction(normal, 0.2);
                    if direction.near_zero() {
                        self.reflective(origin, normal, scene);
                    } else {
                        self.reflective(origin, direction, scene);
                    }
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

    /*
    fn diffuse_reflection_direction(&self, normal: Normal) -> Direction {
        let mut rng = rand::thread_rng();

        // Create a local coordinate system around the normal
        let incident_ray = normal.normalize();
        let tangent_a = if incident_ray.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let tangent_v = incident_ray.cross(&tangent_a).normalize();
        let tangent_u = incident_ray.cross(&tangent_v);

        // Generate random points on a hemisphere
        let rand_1: f64 = rng.gen();
        let rand_2: f64 = rng.gen();
        let sin_theta = (1.0 - rand_2 * rand_2).sqrt();
        let phi = 2.0 * std::f64::consts::PI * rand_1;
        let local_x = phi.cos() * sin_theta;
        let local_y = phi.sin() * sin_theta;
        let local_z = rand_2.sqrt();

        // Convert to world coordinates
        tangent_u * local_x + tangent_v * local_y + incident_ray * local_z
    }

     */

    fn reflective_direction(&self, normal: Normal, diffusion_range: f64) -> Direction {
        // Calculate the perfect reflection direction
        let perfect_reflection = self.direction - 2.0 * self.direction.dot(&normal) * normal;

        // Introduce diffusion by adding a random offset
        let mut rng = rand::thread_rng();
        let random_offset = Ray::cosine_weighted_sample(&normal, &mut rng, diffusion_range);

        // Apply the random offset to the perfect reflection direction
        let diffuse_reflection = perfect_reflection + random_offset;

        diffuse_reflection.normalize() // Normalize the result to ensure it's a valid direction
    }

    // Helper function for cosine-weighted hemisphere sampling
    fn cosine_weighted_sample(
        normal: &Normal,
        rng: &mut impl Rng,
        diffusion_range: f64,
    ) -> Direction {
        // Generate two random numbers in the range [-0.5, 0.5)
        let r1 = rng.gen::<f64>() - 0.5;
        let r2 = rng.gen::<f64>() - 0.5;

        // Scale the random values based on the diffusion range
        let scaled_r1 = r1 * diffusion_range;
        let scaled_r2 = r2 * diffusion_range;

        // Calculate the spherical coordinates based on the scaled random numbers
        let theta = (2.0 * std::f64::consts::PI * scaled_r1).acos().sqrt(); // Polar angle
        let phi = 2.0 * std::f64::consts::PI * scaled_r2; // Azimuthal angle

        // Convert spherical coordinates to Cartesian coordinates
        let x = theta.sin() * phi.cos();
        let y = theta.sin() * phi.sin();
        let z = theta.cos();

        // Transform the sampled direction to the local coordinate system defined by the normal
        // Calculate the tangent vector manually
        let tangent = normal.cross(&Vector3::z());
        let bi_tangent = normal.cross(&tangent);

        // Transform the sampled direction to the local coordinate system defined by the normal
        tangent * x + bi_tangent * y + Vector3::new(normal.x, normal.y, normal.z) * z
    }

    fn reached_max_depth(&self) -> bool {
        self.depth >= MAX_DEPTH
    }
}
