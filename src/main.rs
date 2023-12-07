use rt::gui::launch_gui;
use rt::raytracer::{CameraBuilder, Scene};
use rt::type_aliases::{Direction, Point};
use std::env;
use std::sync::Arc;
use std::time::Instant;
const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--no-gui".to_string()) {
        let mut camera = CameraBuilder::new()
            .sample_size(20)
            .position_by_coordinates(Point::new(-3.0, -4.0, 5.0))
            .look_at(Point::new(0.0, 0.0, 0.0))
            .up_direction_by_coordinates(Direction::new(0.0, 1.0, 0.0))
            .focal_length(0.5)
            .sensor_width(1.0)
            .build();

        let scene_data = "scene info from gui here";
        let scene = Arc::new(Scene::init(scene_data, 0.5));

        let start = Instant::now();

        // Perform ray tracing
        camera.send_rays(scene.clone());

        camera.write_to_ppm(OUTPUT_PATH);

        let duration = start.elapsed();
        println!("Time taken for rendering: {:?}", duration);
    } else {
        launch_gui();
    }
}
