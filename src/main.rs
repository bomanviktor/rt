use rt::raytracer::{CameraBuilder, Scene};
use rt::type_aliases::{Direction, Point};
use std::sync::Arc;
use std::time::Instant;

const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    let mut camera = CameraBuilder::new()
        .sample_size(100)
        .position_by_coordinates(Point::new(-3.0, -4.0, 5.0))
        .look_at(Point::new(0.0, 0.0, 0.0))
        .up_direction_by_coordinates(Direction::new(0.0, 1.0, 0.0))
        .focal_length(0.5)
        .sensor_width(1.0)
        .resolution(1920, 1080)
        .build();

    let scene_data = "scene info from gui here";
    let scene = Arc::new(Scene::init(scene_data, 1.0));
    let start = Instant::now();

    // Perform ray tracing
    camera.send_rays(scene.clone());

    camera.write_to_ppm(OUTPUT_PATH);

    let duration = start.elapsed();
    println!("Time taken for rendering: {:?}", duration);
}
