use rt::raytracer::{CameraBuilder, Scene};
use rt::type_aliases::Point;
use std::sync::Arc;
use std::time::Instant;

const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    let mut camera = CameraBuilder::new()
        .sample_size(100)
        .position_by_coordinates(Point::new(-8.0, 5.0, 13.0))
        .look_at(Point::new(-1.0, 0.5, 0.0))
        .focal_length(1.0)
        .sensor_width(1.0)
        .resolution(1920, 1080)
        .build();

    let scene_data = "scene info from gui here";
    let scene = Arc::new(Scene::init(scene_data, 0.01));
    let start = Instant::now();

    // Perform ray tracing
    camera.send_rays(scene.clone());

    camera.write_to_ppm(OUTPUT_PATH);

    let duration = start.elapsed();
    println!("Time taken for rendering: {:?}", duration);
}
