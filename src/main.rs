// use nalgebra::Vector3;
// use rt::raytracer::{CameraBuilder, Scene};
use rt::gui::launch_gui;

// const OUTPUT_PATH: &str = "output.ppm";

fn main() {
    launch_gui();
    // New camera setup
    // let mut camera = CameraBuilder::new()
    //     .sample_size(1)
    //     .position_by_coordinates(Vector3::new(0.0, -3.0, 2.0))
    //     .look_at(Vector3::new(0.0, 0.0, -5.0))
    //     .up_direction_by_coordinates(Vector3::new(0.0, 4.0, 0.0))
    //     .focal_length(0.5)
    //     .resolution((1600, 900))
    //     .sensor_width(1.0)
    //     .build();
    //
    // let scene_data = "scene info from gui here";
    // let scene = Scene::init(scene_data);
    //
    // // Perform ray tracing
    // camera.send_rays(&scene.objects);
    // camera.write_to_ppm(OUTPUT_PATH);
}
