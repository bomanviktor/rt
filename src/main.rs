use rt::gui::{launch_gui, AppState};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app_state = Rc::new(RefCell::new(AppState {
        spheres: vec![],
        cylinders: vec![],
        cubes: vec![],
        flat_planes: vec![],
    }));
    launch_gui(app_state.clone());
}
