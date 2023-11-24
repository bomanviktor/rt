use glib::clone;
use glib::signal::Inhibit;
use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Button, ComboBoxText, Entry, Orientation, Scale, Separator, Window, WindowType,
};
use std::cell::RefCell;
use std::rc::Rc;

// use crate::objects::sphere;

struct AppState {
    spheres: Vec<SphereConfig>,
    cylinders: Vec<CylinderConfig>,
    cubes: Vec<CubeConfig>,
    flat_planes: Vec<FlatPlaneConfig>,
}
#[allow(dead_code)]
struct ObjectConfig {
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    // material_selector: Rc<RefCell<ComboBoxText>>,
}

struct SphereConfig {
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
}

struct CylinderConfig {
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
    height_entry: Rc<RefCell<Entry>>,
}

struct CubeConfig {
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    size_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
}

struct FlatPlaneConfig {
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
}

pub fn launch_gui() {
    let app_state = Rc::new(RefCell::new(AppState {
        spheres: Vec::new(),
        cylinders: Vec::new(),
        cubes: Vec::new(),
        flat_planes: Vec::new(),
    }));

    gtk::init().expect("Failed to initialize GTK.");

    // // let object_selector = Rc::new(RefCell::new(ComboBoxText::new()));
    // let pos_x_entry = Rc::new(RefCell::new(Entry::new()));
    // let pos_y_entry = Rc::new(RefCell::new(Entry::new()));
    // let pos_z_entry = Rc::new(RefCell::new(Entry::new())); // Added Z position entry
    // let radius_entry = Rc::new(RefCell::new(Entry::new()));
    // // let material_selector = Rc::new(RefCell::new(ComboBoxText::new()));

    // pos_x_entry.borrow().connect_changed(|entry| {
    //     let text = entry.get_text().to_string();
    //     println!("X Entry changed (fn launch_gui): {}", text);
    // });

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Ray Tracing Settings");
    window.set_default_size(600, 600);

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    let vbox_clone = vbox.clone();

    let render_button = Button::with_label("Render picture");
    vbox.pack_start(&render_button, false, false, 0);

    // Create a horizontal box for the side-by-side buttons
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let add_sphere_button = Button::with_label("Add Sphere");
    hbox.pack_start(&add_sphere_button, false, false, 0);

    let add_cylinder_button = Button::with_label("Add Cylinder");
    hbox.pack_start(&add_cylinder_button, false, false, 0);

    let add_cube_button = Button::with_label("Add Cube");
    hbox.pack_start(&add_cube_button, false, false, 0);

    let add_flat_plane_button = Button::with_label("Add Flat Plane");
    hbox.pack_start(&add_flat_plane_button, false, false, 0);

    // Add the horizontal box to the vertical box
    vbox.pack_start(&hbox, false, false, 0);

    // let pos_x_entry = Rc::new(RefCell::new(Entry::new()));

    add_sphere_button.connect_clicked(clone!(@strong vbox_clone, @strong app_state => move |_| {
        let sphere_count = app_state.borrow().spheres.len();
        let sphere_section = create_sphere_section(app_state.clone(), sphere_count + 1);
        vbox_clone.pack_start(&sphere_section, false, false, 0);
        vbox_clone.show_all();
    }));

    add_cylinder_button.connect_clicked(clone!(@strong vbox_clone, @strong app_state => move |_| {
        let cylinder_count = app_state.borrow().spheres.len();
        let cylinder_section = create_cylinder_section(app_state.clone(), cylinder_count + 1);
        vbox_clone.pack_start(&cylinder_section, false, false, 0);
        vbox_clone.show_all();
    }));

    add_cube_button.connect_clicked(clone!(@strong vbox_clone, @strong app_state => move |_| {
    let cube_count = app_state.borrow().cubes.len();
    let cube_section = create_cube_section(app_state.clone(), cube_count + 1);
    vbox_clone.pack_start(&cube_section, false, false, 0);
    vbox_clone.show_all();
    }));

    add_flat_plane_button.connect_clicked(
        clone!(@strong vbox_clone, @strong app_state => move |_| {
        let plane_count = app_state.borrow().flat_planes.len();
        let plane_section = create_flat_plane_section(app_state.clone(), plane_count + 1);
        vbox_clone.pack_start(&plane_section, false, false, 0);
        vbox_clone.show_all();
        }),
    );

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    vbox.pack_start(&separator, false, false, 10);

    let brightness_label = gtk::Label::new(Some("Brightness"));
    vbox.pack_start(&brightness_label, false, false, 0);
    let brightness_entry = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.1);
    vbox.pack_start(&brightness_entry, false, false, 0);

    // Camera Options
    let camera_label = gtk::Label::new(Some("Camera Options"));
    vbox.pack_start(&camera_label, false, false, 0);

    let cam_x_entry = Entry::new();
    cam_x_entry.set_placeholder_text(Some("Camera X Position"));
    vbox.pack_start(&cam_x_entry, false, false, 0);

    let cam_y_entry = Entry::new();
    cam_y_entry.set_placeholder_text(Some("Camera Y Position"));
    vbox.pack_start(&cam_y_entry, false, false, 0);

    let cam_angle_entry = Entry::new();
    cam_angle_entry.set_placeholder_text(Some("Camera Angle"));
    vbox.pack_start(&cam_angle_entry, false, false, 0);

    // Resolution Selection
    let resolution_label = gtk::Label::new(Some("Resolution"));
    vbox.pack_start(&resolution_label, false, false, 0);

    let width_entry = Entry::new();
    width_entry.set_placeholder_text(Some("Width"));
    vbox.pack_start(&width_entry, false, false, 0);

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("Height"));
    vbox.pack_start(&height_entry, false, false, 0);

    // Render Button
    render_button.connect_clicked(clone!(@strong app_state => move |_| {
        let app_state_borrowed = app_state.borrow();

        // Iterate through all spheres and print their properties
        for (index, sphere) in app_state_borrowed.spheres.iter().enumerate() {
            let pos_x = sphere.pos_x_entry.borrow().get_text().to_string();
            let pos_y = sphere.pos_y_entry.borrow().get_text().to_string();
            let pos_z = sphere.pos_z_entry.borrow().get_text().to_string();
            let radius = sphere.radius_entry.borrow().get_text().to_string();
            let material = sphere.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());

            println!("Sphere {}: X: {}, Y: {}, Z: {}, Radius: {}, Material: {}", index + 1, pos_x, pos_y, pos_z, radius, material);
        }

        for (index, cylinder) in app_state_borrowed.cylinders.iter().enumerate() {
            let pos_x = cylinder.pos_x_entry.borrow().get_text().to_string();
            let pos_y = cylinder.pos_y_entry.borrow().get_text().to_string();
            let pos_z = cylinder.pos_z_entry.borrow().get_text().to_string();
            let radius = cylinder.radius_entry.borrow().get_text().to_string();
            let height = cylinder.height_entry.borrow().get_text().to_string();
            let material = cylinder.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());

            println!("Cylinder {}: X: {}, Y: {}, Z: {}, Radius: {}, Height: {}, Material: {}", index + 1, pos_x, pos_y, pos_z, radius, height, material);
        }

        for (index, cube) in app_state_borrowed.cubes.iter().enumerate() {
            let pos_x = cube.pos_x_entry.borrow().get_text().to_string();
            let pos_y = cube.pos_y_entry.borrow().get_text().to_string();
            let pos_z = cube.pos_z_entry.borrow().get_text().to_string();
            let size = cube.size_entry.borrow().get_text().to_string();
            let material = cube.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());

            println!("Cube {}: X: {}, Y: {}, Z: {}, Size: {}, Material: {}", index + 1, pos_x, pos_y, pos_z, size, material);
        }

        for (index, flat_plane) in app_state_borrowed.flat_planes.iter().enumerate() {
            let pos_x = flat_plane.pos_x_entry.borrow().get_text().to_string();
            let pos_y = flat_plane.pos_y_entry.borrow().get_text().to_string();
            let pos_z = flat_plane.pos_z_entry.borrow().get_text().to_string();
            let radius = flat_plane.radius_entry.borrow().get_text().to_string();
            let material = flat_plane.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());

            println!("Flat Plane {}: X: {}, Y: {}, Z: {}, Radius: {}, Material: {}", index + 1, pos_x, pos_y, pos_z, radius, material);
        }


    }));

    window.add(&vbox);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
#[allow(dead_code)]
// // Function to validate and parse position entries
fn validate_and_parse_entry(entry: &gtk::Entry, default_value: f64, label: &str) -> f64 {
    let text = entry.get_text().trim().to_string();
    match text.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!(
                "Error parsing {}: using default value {}",
                label, default_value
            );
            default_value // Use the default value if parsing fails
        }
    }
}

fn create_sphere_section(app_state: Rc<RefCell<AppState>>, sphere_count: usize) -> gtk::Box {
    let sphere_section = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label_text = format!("Sphere {}", sphere_count);
    let sphere_label = gtk::Label::new(Some(&label_text));
    sphere_section.pack_start(&sphere_label, false, false, 0);

    let pos_x_entry = Entry::new();
    pos_x_entry.set_placeholder_text(Some("X Position"));
    sphere_section.pack_start(&pos_x_entry, false, false, 0);

    let pos_y_entry = Entry::new();
    pos_y_entry.set_placeholder_text(Some("Y Position"));
    sphere_section.pack_start(&pos_y_entry, false, false, 0);

    let pos_z_entry = Entry::new();
    pos_z_entry.set_placeholder_text(Some("Z Position"));
    sphere_section.pack_start(&pos_z_entry, false, false, 0);

    let radius_entry = Entry::new();
    radius_entry.set_placeholder_text(Some("Radius"));
    sphere_section.pack_start(&radius_entry, false, false, 0);

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    sphere_section.pack_start(&material_selector, false, false, 0);

    let sphere_config = SphereConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
    };
    app_state.borrow_mut().spheres.push(sphere_config);

    sphere_section
}

fn create_cylinder_section(app_state: Rc<RefCell<AppState>>, cylinder_count: usize) -> gtk::Box {
    let cylinder_section = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label_text = format!("Cylinder {}", cylinder_count);
    let cylinder_label = gtk::Label::new(Some(&label_text));
    cylinder_section.pack_start(&cylinder_label, false, false, 0);

    let pos_x_entry = Entry::new();
    pos_x_entry.set_placeholder_text(Some("X Position"));
    cylinder_section.pack_start(&pos_x_entry, false, false, 0);

    let pos_y_entry = Entry::new();
    pos_y_entry.set_placeholder_text(Some("Y Position"));
    cylinder_section.pack_start(&pos_y_entry, false, false, 0);

    let pos_z_entry = Entry::new();
    pos_z_entry.set_placeholder_text(Some("Z Position"));
    cylinder_section.pack_start(&pos_z_entry, false, false, 0);

    let radius_entry = Entry::new();
    radius_entry.set_placeholder_text(Some("Radius"));
    cylinder_section.pack_start(&radius_entry, false, false, 0);

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("Height"));
    cylinder_section.pack_start(&height_entry, false, false, 0);

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    cylinder_section.pack_start(&material_selector, false, false, 0);

    let cylinder_config = CylinderConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
        height_entry: Rc::new(RefCell::new(height_entry)),
    };
    app_state.borrow_mut().cylinders.push(cylinder_config);

    cylinder_section
}

#[allow(dead_code)]
//Todo: refine this function
fn create_cube_section(app_state: Rc<RefCell<AppState>>, cube_count: usize) -> gtk::Box {
    let cube_section = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label_text = format!("Cube {}", cube_count);
    let cube_label = gtk::Label::new(Some(&label_text));
    cube_section.pack_start(&cube_label, false, false, 0);

    let pos_x_entry = Entry::new();
    pos_x_entry.set_placeholder_text(Some("X Position"));
    cube_section.pack_start(&pos_x_entry, false, false, 0);

    let pos_y_entry = Entry::new();
    pos_y_entry.set_placeholder_text(Some("Y Position"));
    cube_section.pack_start(&pos_y_entry, false, false, 0);

    let pos_z_entry = Entry::new();
    pos_z_entry.set_placeholder_text(Some("Z Position"));
    cube_section.pack_start(&pos_z_entry, false, false, 0);

    let size_entry = Entry::new();
    size_entry.set_placeholder_text(Some("Size"));
    cube_section.pack_start(&size_entry, false, false, 0);

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    cube_section.pack_start(&material_selector, false, false, 0);

    let cube_config = CubeConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        size_entry: Rc::new(RefCell::new(size_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
    };
    app_state.borrow_mut().cubes.push(cube_config);

    cube_section
}

fn create_flat_plane_section(app_state: Rc<RefCell<AppState>>, plane_count: usize) -> gtk::Box {
    let flat_plane_section = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label_text = format!("Flat Plane {}", plane_count);
    let flat_plane_label = gtk::Label::new(Some(&label_text));
    flat_plane_section.pack_start(&flat_plane_label, false, false, 0);

    let pos_x_entry = Entry::new();
    pos_x_entry.set_placeholder_text(Some("X Position"));
    flat_plane_section.pack_start(&pos_x_entry, false, false, 0);

    let pos_y_entry = Entry::new();
    pos_y_entry.set_placeholder_text(Some("Y Position"));
    flat_plane_section.pack_start(&pos_y_entry, false, false, 0);

    let pos_z_entry = Entry::new();
    pos_z_entry.set_placeholder_text(Some("Z Position"));
    flat_plane_section.pack_start(&pos_z_entry, false, false, 0);

    let radius_entry = Entry::new();
    radius_entry.set_placeholder_text(Some("Radius"));
    flat_plane_section.pack_start(&radius_entry, false, false, 0);

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    flat_plane_section.pack_start(&material_selector, false, false, 0);

    let flat_plane_config = FlatPlaneConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
    };
    app_state.borrow_mut().flat_planes.push(flat_plane_config);

    flat_plane_section
}
