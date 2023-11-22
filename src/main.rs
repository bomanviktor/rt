use nalgebra::Vector3;
use rt::raytracer::{CameraBuilder, Scene};

const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    // New camera setup
    let mut camera = CameraBuilder::new()
        .sample_size(1)
        .position_by_coordinates(Vector3::new(0.0, -3.0, 2.0))
        .look_at(Vector3::new(0.0, 0.0, -5.0))
        .up_direction_by_coordinates(Vector3::new(0.0, 4.0, 0.0))
        .focal_length(0.5)
        .resolution((1600, 900))
        .sensor_width(1.0)
        .build();

    let scene_data = "scene info from gui here";
    let scene = Scene::init(scene_data);

    // Perform ray tracing
    camera.send_rays(&scene.objects);

    // Loop through each vector of rays in the camera
    for ray_vector in &camera.rays {
        // Loop through each ray in the vector
        for ray in ray_vector {
            // Check if the collision vector is not empty
            if !ray.collisions.is_empty() {
                // If it's not empty, print each collision color
                for collision in &ray.collisions {
                    println!("Collision color: {:?}", collision);
                }
            }
        }
    }

    camera.write_to_ppm(OUTPUT_PATH);
}
