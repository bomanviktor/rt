use nalgebra::Vector3;
use rt::raytracer::{CameraBuilder, Scene};

const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    // New camera setup
    let mut camera = CameraBuilder::new()
        .sample_size(1)
        .position_by_coordinates(Vector3::new(-6.0, -6.0, 5.0))
        .look_at(Vector3::new(0.0, 0.0, -1.0))
        .up_direction_by_coordinates(Vector3::new(0.0, 3.0, 0.0))
        .focal_length(0.5)
        .resolution((1600, 900))
        .sensor_width(1.0)
        .build();

    let scene_data = "scene info from gui here";
    let scene = Scene::init(scene_data);

    // Perform ray tracing
    camera.send_rays(&scene.objects);
    camera.write_to_ppm(OUTPUT_PATH);
}

// Legacy function for checking intersections with multiple spheres
// TODO: Remove :-(

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
