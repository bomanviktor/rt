use std::sync::Arc;

use nalgebra::Vector3;
use rt::raytracer::{CameraBuilder, Scene};

const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    // New camera setup
    let mut camera = CameraBuilder::new()
        .sample_size(1)
        .position_by_coordinates(Vector3::new(0.0, -3.0, 2.0))
        .look_at(Vector3::new(0.0, 0.0, -5.0))
        .up_direction_by_coordinates(Vector3::new(0.0, 1.0, 0.0))
        .focal_length(0.5)
        .sensor_width(1.0)
        .build();

    let scene_data = "scene info from gui here";
    //let scene = Scene::init(scene_data);
    let scene = Arc::new(Scene::init(scene_data));
    // Perform ray tracing
    camera.send_rays(scene.clone());
    camera.write_to_ppm(OUTPUT_PATH);
}
