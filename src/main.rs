use std::sync::Arc;
use std::time::Instant; // Import Instant for timing

use nalgebra::Vector3;
use rt::raytracer::{CameraBuilder, Scene};

const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    // New camera setup
    let mut camera = CameraBuilder::new()
        .sample_size(20)
        .position_by_coordinates(Vector3::new(-3.0, -4.0, 5.0))
        .look_at(Vector3::new(0.0, 0.0, 0.0))
        .up_direction_by_coordinates(Vector3::new(0.0, 1.0, 0.0))
        .focal_length(1.0)
        .sensor_width(1.0)
        .build();

    let scene_data = "scene info from gui here";
    //let scene = Scene::init(scene_data);
    let scene = Arc::new(Scene::init(scene_data));

    // Start timing
    let start = Instant::now();

    // Perform ray tracing
    camera.send_rays(scene.clone());
    camera.write_to_ppm(OUTPUT_PATH);

    // Stop timing and calculate duration
    let duration = start.elapsed();

    // Print the time taken
    println!("Time taken for rendering: {:?}", duration);
}
