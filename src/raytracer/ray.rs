use std::sync::Arc;

use crate::config::Point;
use crate::objects::{Intersection, Object};
use crate::objects::{Objects, Texture};
use crate::raytracer::Scene;
use nalgebra::Vector3;
use rand::Rng;

const MAX_DEPTH: u8 = 5;
const NUM_SECONDARY_RAYS: usize = 4;
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
    pub collisions: Vec<Vector3<f64>>,
    pub hit_light_source: bool,
    pub intersection_dist: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            collisions: Vec::new(),
            hit_light_source: false,
            intersection_dist: f64::MAX,
        }
    }

    pub fn trace(&mut self, scene: &Scene, depth: u8, rng: &mut impl Rng) {
        let new_rays = NUM_SECONDARY_RAYS / 2_usize.pow(depth as u32);
        if depth >= MAX_DEPTH || new_rays == 0 {
            return; // Stop if maximum depth is reached
        }

        let mut closest_intersection: Intersection = None;
        let mut closest_object: Option<Arc<dyn Object>> = None; // To keep track of the object with the closest intersection

        // Check for intersection with objects
        for object in &scene.objects {
            if let Some((hit_point, distance)) = object.intersection(self) {
                if distance < self.intersection_dist {
                    self.intersection_dist = distance;
                    closest_intersection = Some((hit_point, distance));
                    closest_object = Some(object.clone());
                }
            }
        }

        // Process the closest intersection
        if let Some(intersection) = closest_intersection {
            if let Some(object) = closest_object {
                // Check if the intersection is in shadow
                // Take this out for now, will implement in next PR for normal shading.
                /*
                let in_shadow = Ray::in_shadow(
                    intersection.0,
                    object.normal_at(self, intersection.0),
                    &scene.objects,
                    object.center(),
                );
                 */

                self.collisions.push(object.color());
                match object.texture() {
                    Texture::Diffusive => {
                        self.diffuse(Some(intersection), object, new_rays, scene, depth, rng);
                    }
                    Texture::Glossy => {
                        unimplemented!()
                        // self.glossy(intersection, object, new_rays, scene, depth);
                    }
                    Texture::Reflective => {
                        unimplemented!()
                        // self.reflective(intersection, object, new_rays, scene, depth);
                    }
                    Texture::Light => {
                        self.hit_light_source = true;
                    }
                }
            }
        }
    }

    fn diffuse(
        &mut self,
        intersection: Intersection,
        object: Arc<dyn Object>,
        new_rays: usize,
        scene: &Scene,
        depth: u8,
        rng: &mut impl Rng,
    ) {
        let first_hit_point = intersection.unwrap().0;

        // Iterate over secondary rays
        for _ in 0..new_rays {
            let new_direction =
                self.diffuse_direction(object.normal_at(self, first_hit_point), rng);

            let mut secondary_ray = Ray {
                origin: first_hit_point,
                direction: new_direction,
                collisions: self.collisions.clone(),
                hit_light_source: false,
                intersection_dist: f64::MAX,
            };

            // Recursively trace the secondary ray
            secondary_ray.trace(scene, depth + 1, rng);
            // Accumulate colors from secondary rays into the original ray's collisions
            // also set the hit_light_source to true if the secondary ray hit a light source
            if secondary_ray.hit_light_source {
                self.hit_light_source = true;
            }
            self.collisions.extend(secondary_ray.collisions);
        }
    }

    fn diffuse_direction(&self, normal: Vector3<f64>, rng: &mut impl Rng) -> Vector3<f64> {
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
    #[allow(dead_code)]
    fn reflect(&self, normal: Vector3<f64>) -> Vector3<f64> {
        self.direction - 2.0 * self.direction.dot(&normal) * normal
    }

    pub fn average_color(&self) -> Vector3<f64> {
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
            (total / number_of_colors) * 5.0
        } else {
            (total / number_of_colors) * 0.2
        }
    }

    pub fn in_shadow(
        hit_point: Vector3<f64>,
        normal: Vector3<f64>,
        objects: &Objects,
        object_center: Point,
    ) -> bool {
        let light_source = objects.iter().find(|obj| obj.is_light()).cloned().unwrap();
        let light_position = light_source.center();

        let to_light = (light_position - hit_point).normalize();
        let shadow_ray = Ray::new(hit_point + normal * 1e-1, to_light);

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
}
