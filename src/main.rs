use nalgebra::Vector3;
use rt::color::Color;
use rt::objects::Object;
use rt::objects::{Cylinder, Sphere};
use rt::raytracer::{CameraBuilder, Ray};
use std::io::Write;

fn main() {
    // New camera setup
    let camera = CameraBuilder::new()
        .sample_size(1)
        .position_by_coordinates(Vector3::new(-6.0, -6.0, 5.0))
        .look_at(Vector3::new(0.0, 0.0, -1.0))
        .up_direction_by_coordinates(Vector3::new(0.0, 3.0, 0.0))
        //.aspect_ratio(16.0 / 9.0)
        .focal_length(0.5)
        .resolution((1600, 900))
        .sensor_width(1.0)
        .build();

    // Objects setup
    let sphere1 = Sphere::new(Vector3::new(0.0, 0.0, -5.0), 1.0, Color::new(255, 0, 0));
    let sphere2 = Sphere::new(Vector3::new(2.0, 0.0, -5.0), 1.2, Color::new(255, 0, 0));
    let sphere3 = Sphere::new(Vector3::new(-2.0, 0.0, -5.0), 1.2, Color::new(255, 0, 0));
    let sphere4 = Sphere::new(Vector3::new(0.0, -2.0, -5.0), 1.0, Color::new(255, 0, 0));
    let sphere5 = Sphere::new(Vector3::new(0.0, -4.0, -5.0), 1.0, Color::new(255, 0, 0));
    let sphere6 = Sphere::new(Vector3::new(0.0, 6.0, -5.0), 1.5, Color::new(255, 0, 0));

    let cylinder = Cylinder::new(
        Vector3::new(0.0, -2.0, -2.0),
        1.5,
        4.0,
        Color::new(0, 255, 0),
    );

    // Collect objects
    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3),
        Box::new(sphere4),
        Box::new(sphere5),
        Box::new(sphere6),
        Box::new(cylinder),
    ];

    // Perform ray tracing
    let pixel_data = check_intersections_with_multiple_objects(&camera.rays, &objects);
    write_to_ppm(&pixel_data, "output.ppm");
}

fn write_to_ppm(pixels: &Vec<Vec<Color>>, filename: &str) {
    let mut file = std::fs::File::create(filename).unwrap();
    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", pixels[0].len(), pixels.len()).unwrap();
    writeln!(file, "255").unwrap();
    for row in pixels {
        for pixel in row {
            writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
        }
    }
}

// Legacy function for checking intersections with multiple spheres

// fn check_intersections_with_multiple_spheres(
//     rays: &Vec<Vec<Ray>>,
//     spheres: &Vec<Sphere>,
// ) -> Vec<Vec<Color>> {
//     let mut pixels = Vec::new();
//     for row in rays {
//         let mut pixel_row = Vec::new();
//         for ray in row {
//             let mut closest_intersection: Option<(Vector3<f64>, f64)> = None; // Track point and distance
//             let mut closest_sphere: Option<&Sphere> = None;
//
//             for sphere in spheres {
//                 if let Some((hit_point, distance)) = sphere.intersection(ray) {
//                     if closest_intersection.is_none() || distance < closest_intersection.unwrap().1
//                     {
//                         closest_intersection = Some((hit_point, distance));
//                         closest_sphere = Some(sphere);
//                     }
//                 }
//             }
//
//             if let Some(sphere) = closest_sphere {
//                 let hit_point = closest_intersection.unwrap().0;
//                 let normal = sphere.normal_at(hit_point);
//                 let modified_color =
//                     modify_color_based_on_normal(normal, sphere.color.clone(), ray);
//                 pixel_row.push(modified_color);
//             } else {
//                 pixel_row.push(Color::default()); // Background color
//             }
//         }
//         pixels.push(pixel_row);
//     }
//     pixels
// }

fn check_intersections_with_multiple_objects(
    rays: &Vec<Vec<Ray>>,
    objects: &Vec<Box<dyn Object>>,
) -> Vec<Vec<Color>> {
    let mut pixels = Vec::new();
    for row in rays {
        let mut pixel_row = Vec::new();
        for ray in row {
            let mut closest_intersection: Option<(Vector3<f64>, f64)> = None;
            let mut closest_object: Option<&Box<dyn Object>> = None;

            for object in objects {
                if let Some((hit_point, distance)) = object.intersection(ray) {
                    if closest_intersection.is_none() || distance < closest_intersection.unwrap().1
                    {
                        closest_intersection = Some((hit_point, distance));
                        closest_object = Some(object);
                    }
                }
            }

            if let Some(object) = closest_object {
                let hit_point = closest_intersection.unwrap().0;
                let normal = object.normal_at(hit_point);
                let modified_color = modify_color_based_on_normal(normal, object.color(), ray);
                pixel_row.push(modified_color);
            } else {
                pixel_row.push(Color::default());
            }
        }
        pixels.push(pixel_row);
    }
    pixels
}

fn modify_color_based_on_normal(normal: Vector3<f64>, original_color: Color, ray: &Ray) -> Color {
    let dot = normal.dot(&ray.direction.normalize()).abs();
    Color::new(
        (original_color.r as f64 * dot) as u8,
        (original_color.g as f64 * dot) as u8,
        (original_color.b as f64 * dot) as u8,
    )
}
