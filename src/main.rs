use rt::gui::{launch_gui, AppState};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Shared state between GUI and main logic
    // Initialize AppState with empty vectors
    let app_state = Rc::new(RefCell::new(AppState {
        spheres: vec![],
        cylinders: vec![],
        cubes: vec![],
        flat_plane: vec![],
    }));

    // Pass AppState to launch_gui
    launch_gui(app_state.clone());
}