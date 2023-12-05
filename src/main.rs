use std::sync::Arc;
use std::time::Instant;

use nalgebra::Vector3;
use rt::raytracer::{CameraBuilder, Scene};

//new main function to generat 100 ppms output1-output100:

fn main() {
    for i in 1..101 {
        let mut camera = CameraBuilder::new()
            .sample_size(200)
            .position_by_coordinates(Vector3::new(-3.0, -4.0, 4.0))
            .look_at(Vector3::new(0.0, 0.0, 0.0))
            .up_direction_by_coordinates(Vector3::new(0.0, 1.0, 0.0))
            .focal_length(0.5)
            .sensor_width(1.0)
            .build();
        let out = format!("outputreflect{}.ppm", i);

        let scene_data = "scene info from gui here";
        let scene = Arc::new(Scene::init(scene_data));

        let start = Instant::now();

        // Perform ray tracing
        camera.send_rays(scene.clone());

        camera.write_to_ppm(&out);

        let duration = start.elapsed();
        println!("Time taken for rendering: {:?}", duration);
    }
}
