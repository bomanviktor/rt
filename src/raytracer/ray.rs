use crate::color::Color;
use crate::config::Point;
use crate::objects::Object;
use crate::objects::Texture;
use crate::raytracer::Scene;
use nalgebra::Vector3;
use rand::Rng;

const NUM_SECONDARY_RAYS: usize = 5;
const SMALL_OFFSET: f64 = 0.001;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>,
    pub collisions: Vec<Color>,
    pub hit_light_source: bool,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self {
            origin,
            direction,
            collisions: Vec::new(),
            hit_light_source: false,
        }
    }

    pub fn trace(&mut self, scene: &Scene, depth: u32) {
        // if depth >= 10 {
        //     return; // Stop if maximum depth is reached
        // }
        ////////////////////////////////////////////////////////////////////////////////////
        // println!(
        //     "Tracing ray: Origin = {:?}, Direction = {:?}",
        //     ray.origin, ray.direction
        // );

        let mut closest_intersection: Option<(Vector3<f64>, f64)> = None;
        let mut closest_object: Option<&Box<dyn Object>> = None;

        // Check for intersection with objects
        for object in &scene.objects {
            if let Some((hit_point, distance)) = object.intersection(self) {
                if closest_intersection.is_none() || distance < closest_intersection.unwrap().1 {
                    closest_intersection = Some((hit_point, distance));
                    closest_object = Some(object);

                    let mut color = object.color();

                    // Check if the object's texture is Light
                    if matches!(object.texture(), Texture::Light) {
                        self.hit_light_source = true;
                        self.collisions.push(color);
                        return;
                    } else {
                        color = self
                            .modify_color_based_on_normal(object.normal_at(self, hit_point), color);
                        self.collisions.push(color);
                    }
                }
            }
        }

        if let Some(object) = closest_object {
            let first_hit_point = closest_intersection.unwrap().0;

            // Iterate over secondary rays
            for _ in 0..NUM_SECONDARY_RAYS {
                if depth + 1 >= 5 {
                    continue; // Limit the depth of secondary rays to 5 bounces
                }

                let new_direction =
                    self.generate_new_direction(object.normal_at(self, first_hit_point));

                let mut secondary_ray = Ray {
                    origin: first_hit_point + new_direction * SMALL_OFFSET,
                    direction: new_direction,
                    collisions: Vec::new(),
                    hit_light_source: false,
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
    }
    fn modify_color_based_on_normal(&self, normal: Vector3<f64>, original_color: Color) -> Color {
        let dot = normal.dot(&self.direction.normalize()).abs();
        Color::new(
            (original_color.r as f64 * dot) as u8,
            (original_color.g as f64 * dot) as u8,
            (original_color.b as f64 * dot) as u8,
        )
    }

    pub fn average_color(&self) -> Color {
        if self.collisions.len() == 1 {
            return self.collisions[0];
        }

        let primary_color = self.collisions[0];
        let secondary_colors = &self.collisions[1..];
        let light_source_boost = 1.0;

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

    fn reflect(&self, normal: Vector3<f64>) -> Vector3<f64> {
        self.direction - 2.0 * self.direction.dot(&normal) * normal
    }

    fn generate_new_direction(&self, normal: Vector3<f64>) -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        // Create a coordinate system around the normal
        let w = normal.normalize();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
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
        let new_dir = u * x + v * y + w * z;
        new_dir * new_dir.dotc(&self.reflect(normal))
    }
}
