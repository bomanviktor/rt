use rt::gui::launch_gui;
use rt::raytracer::{CameraBuilder, Scene};
use rt::type_aliases::Point;
use std::env;
use std::sync::Arc;
use std::time::Instant;
const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"no-gui".to_string()) {
        let mut camera = CameraBuilder::new()
            .sample_size(1000)
            .position_by_coordinates(Point::new(-6.0, 4.0, 15.0))
            .look_at(Point::new(0.0, 0.0, 0.0))
            .focal_length(1.0)
            .build();

        let scene = Arc::new(Scene::init(0.01));

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
