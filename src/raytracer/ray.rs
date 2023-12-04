use crate::objects::{Intersection, Texture};
use crate::raytracer::Scene;
use crate::type_aliases::{Color, Direction, Normal, Point};
use nalgebra::Vector3;
use rand::Rng;

const MAX_DEPTH: u8 = 50;
const NUM_SECONDARY_RAYS: usize = 8;
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

        let mut closest_intersection: Option<Intersection> = None;
        let mut distance = f64::MAX;
        // Check for intersection with objects
        for object in &scene.objects {
            if let Some(intersection) = object.intersection(self) {
                if intersection.distance < distance {
                    distance = intersection.distance;
                    closest_intersection = Some(intersection);
                }
            }
        }

        // Process the closest intersection
        if let Some(intersection) = closest_intersection {
            let origin = intersection.hit_point;
            let normal = intersection.normal;

            match intersection.texture {
                Texture::Diffusive(color) => {
                    let direction = self.diffuse(normal);
                    self.collisions.push(color);
                    self.diffusive(origin, direction, scene);
                }
                Texture::Glossy(color) => {
                    let reflective_direction = self.reflect(normal);
                    let diffusive_direction = self.diffuse(normal);
                    let glossy_direction =
                        reflective_direction.dot(&diffusive_direction) * diffusive_direction;
                    self.collisions.push(color);
                    self.diffusive(origin, glossy_direction, scene);
                }
                Texture::Reflective => {
                    let direction = self.reflect(normal);
                    self.reflective(origin, direction, scene);
                }
                Texture::Light(color) => {
                    self.collisions.push(color);
                    self.hit_light_source = true;
                }
            }
        }
    }

    fn diffusive(&mut self, origin: Point, direction: Direction, scene: &Scene) {
        let new_rays = match self.depth {
            0 => NUM_SECONDARY_RAYS,
            1 => NUM_SECONDARY_RAYS / 2,
            2 => NUM_SECONDARY_RAYS / 4,
            _ => 1,
        };

        // Iterate over secondary rays
        for _ in 0..new_rays {
            let mut secondary_ray = Ray::new(origin, direction, self.depth + 1);

            // Recursively trace the secondary ray
            secondary_ray.trace(scene);
            // Accumulate colors from secondary rays into the original ray's collisions
            // also set the hit_light_source to true if the secondary ray hit a light source
            if secondary_ray.hit_light_source {
                self.hit_light_source = true;
            }
            self.collisions.extend(secondary_ray.collisions);
        }
    }

    fn diffuse(&self, normal: Normal) -> Direction {
        let mut rng = rand::thread_rng();

        // Create a coordinate system around the normal
        let w = normal.normalize();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(-1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);

        // Generate random points on a hemisphere
        let r1: f64 = rng.gen();
        let r2: f64 = rng.gen();
        let sin_theta = (1.0 - r2 * r2).sqrt();
        let phi = 2.0 * std::f64::consts::PI * r1;
        let x = phi.cos() * sin_theta;
        let y = phi.sin() * sin_theta;
        let z = r2.sqrt();

        // Convert to world coordinates
        u * x + v * y + w * z
    }

    // TODO: remove the macro
    fn reflect(&self, normal: Normal) -> Normal {
        self.direction - 2.0 * self.direction.dot(&normal) * normal
    }

    fn reflective(&mut self, origin: Point, direction: Direction, scene: &Scene) {
        let mut secondary_ray = Ray::new(origin, direction, self.depth + 1);
        secondary_ray.trace(scene);

        if secondary_ray.hit_light_source {
            self.hit_light_source = true;
        }
        self.collisions.extend(secondary_ray.collisions);
    }

    pub fn average_color(&self) -> Color {
        if self.collisions.len() == 1 {
            return self.collisions[0];
        }

        let primary_color = self.collisions[0];
        let secondary_colors = &self.collisions[1..];

        let mut total = primary_color;

        for (i, color) in secondary_colors.iter().enumerate() {
            let mul = 1.0 / (i as f64 + 1.0); // Change this to be according to max depth
            total += color * mul;
        }
        let number_of_colors = self.collisions.len() as f64;
        if self.hit_light_source {
            (total / number_of_colors) * 3.0
        } else {
            (total / number_of_colors) * 0.01
        }
    }
    /*
       pub fn in_shadow(
           hit_point: Vector3<f64>,
           normal: Vector3<f64>,
           objects: &Objects,
           object_center: Point,
       ) -> bool {
           let light_source = objects.iter().find(|obj| obj.is_light()).cloned().unwrap();
           let light_position = light_source.center();

           let to_light = (light_position - hit_point).normalize();
           let shadow_ray = Ray::new(hit_point + normal * 1e-1, to_light, 1);

           for object in objects
               .iter()
               .filter(|&obj| !obj.is_light() && obj.center() != object_center)
           {
               if let Some((shadow_hit, _)) = object.intersection(&shadow_ray) {
                   let distance_to_light = (light_position - shadow_hit).norm();
                   let original_distance_to_light = (light_position - hit_point).norm();

                   if distance_to_light < original_distance_to_light {
                       return true;
                   }
               }
           }
           false
       }
    */

    // TODO: remove the macro
    #[allow(dead_code)]
    fn modify_color_based_on_normal(
        &self,
        normal: Vector3<f64>,
        original_color: Vector3<f64>,
    ) -> Vector3<f64> {
        let dot = normal.dot(&self.direction.normalize()).abs();
        original_color * dot
    }

    fn reached_max_depth(&self) -> bool {
        self.depth >= MAX_DEPTH
    }
}
