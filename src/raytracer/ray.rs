use std::sync::Arc;

use crate::color::Color;
use crate::config::Point;
use crate::objects::{Intersection, Object};
use crate::objects::{Objects, Texture};
use crate::raytracer::Scene;
use nalgebra::Vector3;
use rand::Rng;

const MAX_DEPTH: u8 = 5;
const NUM_SECONDARY_RAYS: usize = 10;
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
    pub collisions: Vec<Color>,
    pub hit_light_source: bool,
    pub closest_intersection_distance: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            collisions: Vec::new(),
            hit_light_source: false,
            closest_intersection_distance: std::f64::MAX,
        }
    }
    pub fn update_closest_intersection(&mut self, distance: f64) {
        if self.closest_intersection_distance < 0.0 || distance < self.closest_intersection_distance
        {
            self.closest_intersection_distance = distance;
        }
    }

    pub fn trace(&mut self, scene: &Scene, depth: u8) {
        self.closest_intersection_distance = f64::INFINITY; // Initialize with infinity
        let new_rays = NUM_SECONDARY_RAYS / 2_u8.pow(depth as u32) as usize;
        if depth >= MAX_DEPTH || new_rays == 0 {
            return; // Stop if maximum depth is reached
        }

        let mut closest_intersection: Intersection = None;
        let mut closest_object: Option<Arc<dyn Object>> = None; // To keep track of the object with the closest intersection

        // Check for intersection with objects
        for object in &scene.objects {
            if let Some((hit_point, distance)) = object.intersection(self) {
                if distance < self.closest_intersection_distance {
                    self.closest_intersection_distance = distance;
                    closest_intersection = Some((hit_point, distance));
                    closest_object = Some(object.clone());
                }
            }
        }

        // Process the closest intersection
        if let Some(intersection) = closest_intersection {
            if let Some(object) = closest_object {
                // Check if the intersection is in shadow
                let in_shadow = Ray::is_in_shadow(
                    intersection.0,
                    object.normal_at(self, intersection.0),
                    &scene.objects,
                    scene,
                    object.color(),
                );
                let mut color = object.color();

                // If in shadow, dim the color
                if in_shadow {
                    let dimming_factor = 0.1; // Adjust as needed
                    color = Color::new(
                        (color.r as f64 * dimming_factor) as u8,
                        (color.g as f64 * dimming_factor) as u8,
                        (color.b as f64 * dimming_factor) as u8,
                    );
                }

                self.collisions.push(color);
                match object.texture() {
                    Texture::Diffusive => {
                        self.diffuse(Some(intersection), object, new_rays, scene, depth);
                    }
                    Texture::Glossy => {
                        // Implement glossy behavior
                        // self.glossy(intersection, object, new_rays, scene, depth);
                    }
                    Texture::Reflective => {
                        // Implement reflective behavior
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
    ) {
        let first_hit_point = intersection.unwrap().0;

        // Iterate over secondary rays
        for _ in 0..new_rays {
            let new_direction = self.diffuse_direction(object.normal_at(self, first_hit_point));

            let mut secondary_ray = Ray {
                origin: first_hit_point * (1.0 + f64::EPSILON),
                direction: new_direction,
                collisions: self.collisions.clone(),
                hit_light_source: false,
                closest_intersection_distance: -1.0,
            };

            // Recursively trace the secondary ray
            secondary_ray.trace(scene, depth + 1);
            // Accumulate colors from secondary rays into the original ray's collisions
            // also set the hit_light_source to true if the secondary ray hit a light source
            if secondary_ray.hit_light_source {
                self.collisions.extend(secondary_ray.collisions);
                self.hit_light_source = true;
            }
        }
    }

    fn diffuse_direction(&self, normal: Vector3<f64>) -> Vector3<f64> {
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
    #[allow(dead_code)]
    fn reflect(&self, normal: Vector3<f64>) -> Vector3<f64> {
        self.direction - 2.0 * self.direction.dot(&normal) * normal
    }

    pub fn average_color(&self) -> Color {
        if self.collisions.len() == 1 {
            return self.collisions[0];
        }

        let primary_color = self.collisions[0];
        let secondary_colors = &self.collisions[1..];
        let light_source_boost = 1.5;

        let primary_weight = 0.8;
        let secondary_weight = 0.2;

        let mut total_r = (primary_color.r as f64 * primary_weight) as u32;
        let mut total_g = (primary_color.g as f64 * primary_weight) as u32;
        let mut total_b = (primary_color.b as f64 * primary_weight) as u32;

        let number_of_colors = secondary_colors.len() as f64;
        let secondary_weight_per_color = secondary_weight / number_of_colors;

        for color in secondary_colors {
            total_r += (color.r as f64 * secondary_weight_per_color * light_source_boost) as u32;
            total_g += (color.g as f64 * secondary_weight_per_color * light_source_boost) as u32;
            total_b += (color.b as f64 * secondary_weight_per_color * light_source_boost) as u32;
        }

        Color {
            r: (total_r as f64 / (primary_weight + secondary_weight)) as u8,
            g: (total_g as f64 / (primary_weight + secondary_weight)) as u8,
            b: (total_b as f64 / (primary_weight + secondary_weight)) as u8,
        }
    }

    pub fn is_in_shadow(
        hit_point: Vector3<f64>,
        normal: Vector3<f64>,
        objects: &Objects,
        scene: &Scene,
        originating_object_color: Color,
    ) -> bool {
        scene.light_sources.iter().any(|light_source| {
            let light_position = light_source.position();
            let to_light = (light_position - hit_point).normalize();
            let shadow_ray = Ray::new(hit_point + normal * 1e-1, to_light); // Adjusted epsilon

            objects.iter().any(|obj| {
                if obj.texture() == Texture::Light {
                    false
                } else if obj.color() != originating_object_color {
                    if let Some((shadow_hit, _)) = obj.intersection(&shadow_ray) {
                        let distance_to_light = (light_position - shadow_hit).norm();
                        let original_distance_to_light = (light_position - hit_point).norm();
                        distance_to_light < original_distance_to_light
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
        })
    }

    // TODO: remove the macro
    #[allow(dead_code)]
    fn modify_color_based_on_normal(&self, normal: Vector3<f64>, original_color: Color) -> Color {
        let dot = normal.dot(&self.direction.normalize()).abs();
        Color::new(
            (original_color.r as f64 * dot) as u8,
            (original_color.g as f64 * dot) as u8,
            (original_color.b as f64 * dot) as u8,
        )
    }
}
