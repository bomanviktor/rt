use glib::clone;
use glib::signal::Inhibit;
use gtk::prelude::*;
use gtk::{
    Box as GtkBox, Button, ComboBoxText, CssProvider, Entry, Orientation, Scale, Separator, Window,
    WindowType,
};
use std::cell::RefCell;
use std::rc::Rc;

struct AppState {
    spheres: Vec<SphereConfig>,
    cylinders: Vec<CylinderConfig>,
    cubes: Vec<CubeConfig>,
    flat_plane: Vec<FlatPlaneConfig>,
}
#[allow(dead_code)]
struct ObjectConfig {
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
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
    radius_entry: Rc<RefCell<Entry>>,
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
        flat_plane: Vec::new(),
    }));

    gtk::init().expect("Failed to initialize GTK.");

    // Create a CSS provider
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Ray Tracing Settings");
    window.set_default_size(900, 900);
    window
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_border_width(10);
    vbox.set_spacing(10);

    let vbox_clone = vbox.clone();

    scrolled_window.add(&vbox);
    window.add(&scrolled_window);

    let render_button = Button::with_label("Render picture");
    vbox.pack_start(&render_button, false, false, 0);
    render_button
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Create a horizontal box for the side-by-side buttons
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let add_sphere_button = Button::with_label("Add Sphere");
    hbox.pack_start(&add_sphere_button, false, false, 0);
    add_sphere_button
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let add_cylinder_button = Button::with_label("Add Cylinder");
    hbox.pack_start(&add_cylinder_button, false, false, 0);
    add_cylinder_button
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let add_cube_button = Button::with_label("Add Cube");
    hbox.pack_start(&add_cube_button, false, false, 0);
    add_cube_button
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let add_flat_plane_button = Button::with_label("Add Flat Plane");
    hbox.pack_start(&add_flat_plane_button, false, false, 0);
    add_flat_plane_button
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Add the horizontal box to the vertical box
    vbox.pack_start(&hbox, false, false, 0);

    let flow_box = gtk::FlowBox::new();
    flow_box.set_valign(gtk::Align::Start);
    flow_box.set_max_children_per_line(10); // Adjust as needed
    flow_box.set_selection_mode(gtk::SelectionMode::None);

    add_sphere_button.connect_clicked(clone!(@strong flow_box, @strong app_state => move |_| {
        let sphere_count = app_state.borrow().spheres.len();
        let sphere_section = create_sphere_section(app_state.clone(), sphere_count + 1);
        flow_box.add(&sphere_section);
        flow_box.show_all();
    }));

    add_cylinder_button.connect_clicked(clone!(@strong flow_box, @strong app_state => move |_| {
        let cylinder_count = app_state.borrow().cylinders.len();
        let cylinder_section = create_cylinder_section(app_state.clone(), cylinder_count + 1);
        flow_box.add(&cylinder_section);
        flow_box.show_all();
    }));

    add_cube_button.connect_clicked(clone!(@strong vbox_clone, @strong app_state => move |_| {
        let cube_count = app_state.borrow().cubes.len();
        let cube_section = create_cube_section(app_state.clone(), cube_count + 1);
        vbox_clone.pack_start(&cube_section, false, false, 0);
        vbox_clone.show_all();
    }));

    add_flat_plane_button.connect_clicked(clone!(@strong vbox_clone, @strong app_state => move |_| {
        let flat_plane_count = app_state.borrow().flat_plane.len();
        let flat_plane_section = create_flat_plane_section(app_state.clone(), flat_plane_count + 1);
        vbox_clone.pack_start(&flat_plane_section, false, false, 0);
        vbox_clone.show_all();
    }));

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
    vbox.pack_start(&flow_box, false, false, 0);

    let brightness_entry_clone = brightness_entry.clone();
    let cam_x_entry_clone = cam_x_entry.clone();
    let cam_y_entry_clone = cam_y_entry.clone();
    let cam_angle_entry_clone = cam_angle_entry.clone();
    let width_entry_clone = width_entry.clone();
    let height_entry_clone = height_entry.clone();

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
            let radius = cube.radius_entry.borrow().get_text().to_string();
            let material = cube.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());

            println!("Cube {}: X: {}, Y: {}, Z: {}, Radius: {}, Material: {}", index + 1, pos_x, pos_y, pos_z, radius, material);
        }

        for (index, flat_plane) in app_state_borrowed.flat_plane.iter().enumerate() {
            let pos_x = flat_plane.pos_x_entry.borrow().get_text().to_string();
            let pos_y = flat_plane.pos_y_entry.borrow().get_text().to_string();
            let pos_z = flat_plane.pos_z_entry.borrow().get_text().to_string();
            let radius = flat_plane.radius_entry.borrow().get_text().to_string();
            let material = flat_plane.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());

            println!("Flat Plane {}: X: {}, Y: {}, Z: {}, Radius: {}, Material: {}", index + 1, pos_x, pos_y, pos_z, radius, material);
        }
        println!("Brightness: {}", brightness_entry_clone.get_value());
        println!("Camera X Position: {}", cam_x_entry_clone.get_text());
        println!("Camera Y Position: {}", cam_y_entry_clone.get_text());
        println!("Camera Angle: {}", cam_angle_entry_clone.get_text());
        println!("Resolution Width: {}", width_entry_clone.get_text());
        println!("Resolution Height: {}", height_entry_clone.get_text());


    }));

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

fn create_sphere_section(app_state: Rc<RefCell<AppState>>, sphere_count: usize) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let grid = gtk::Grid::new();
    grid.set_column_spacing(10); // Adjust the spacing as needed

    let label_text = format!("Sphere {}", sphere_count);
    let sphere_label = gtk::Label::new(Some(&label_text));
    grid.attach(&sphere_label, 0, 0, 1, 1); // Column 0, Row 0

    let pos_x_entry = Entry::new();
    pos_x_entry.set_placeholder_text(Some("X Position"));
    grid.attach(&pos_x_entry, 1, 0, 1, 1); // Column 1, Row 0

    let pos_y_entry = Entry::new();
    pos_y_entry.set_placeholder_text(Some("Y Position"));
    grid.attach(&pos_y_entry, 1, 1, 1, 1); // Column 1, Row 1

    let pos_z_entry = Entry::new();
    pos_z_entry.set_placeholder_text(Some("Z Position"));
    grid.attach(&pos_z_entry, 1, 2, 1, 1); // Column 1, Row 2

    let radius_entry = Entry::new();
    radius_entry.set_placeholder_text(Some("Radius"));
    grid.attach(&radius_entry, 1, 3, 1, 1); // Column 1, Row 3

    // Apply styles to entries
    let entries = vec![&pos_x_entry, &pos_y_entry, &pos_z_entry, &radius_entry];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    grid.attach(&material_selector, 1, 4, 1, 1); // Column 1, Row 4

    // Apply styles to ComboBoxText
    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    let sphere_config = SphereConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
    };
    app_state.borrow_mut().spheres.push(sphere_config);

    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

fn create_cylinder_section(app_state: Rc<RefCell<AppState>>, cylinder_count: usize) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let grid = gtk::Grid::new();
    grid.set_column_spacing(10); // Adjust the spacing as needed

    let label_text = format!("Cylinder {}", cylinder_count);
    let cylinder_label = gtk::Label::new(Some(&label_text));
    grid.attach(&cylinder_label, 0, 0, 1, 1); // Column 0, Row 0

    let pos_x_entry = Entry::new();
    pos_x_entry.set_placeholder_text(Some("X Position"));
    grid.attach(&pos_x_entry, 1, 0, 1, 1); // Column 1, Row 0

    let pos_y_entry = Entry::new();
    pos_y_entry.set_placeholder_text(Some("Y Position"));
    grid.attach(&pos_y_entry, 1, 1, 1, 1); // Column 1, Row 1

    let pos_z_entry = Entry::new();
    pos_z_entry.set_placeholder_text(Some("Z Position"));
    grid.attach(&pos_z_entry, 1, 2, 1, 1); // Column 1, Row 2

    let radius_entry = Entry::new();
    radius_entry.set_placeholder_text(Some("Radius"));
    grid.attach(&radius_entry, 1, 3, 1, 1); // Column 1, Row 3

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("Height"));
    grid.attach(&height_entry, 1, 4, 1, 1); // Column 1, Row 4

    // Apply styles to entries
    let entries = vec![
        &pos_x_entry,
        &pos_y_entry,
        &pos_z_entry,
        &radius_entry,
        &height_entry,
    ];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    grid.attach(&material_selector, 1, 5, 1, 1); // Column 1, Row 5

    // Apply styles to ComboBoxText
    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    let cylinder_config = CylinderConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
        height_entry: Rc::new(RefCell::new(height_entry)),
    };
    app_state.borrow_mut().cylinders.push(cylinder_config);

    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

//Todo: refine this function
fn create_cube_section(app_state: Rc<RefCell<AppState>>, cube_count: usize) -> gtk::Box {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let cube_section = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label_text = format!("Cube {}", cube_count);
    let cube_label = gtk::Label::new(Some(&label_text));
    cube_section.pack_start(&cube_label, false, false, 0);
    let style_context = cube_label.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let pos_x_entry = Entry::new();
    pos_x_entry.set_placeholder_text(Some("X Position"));
    cube_section.pack_start(&pos_x_entry, false, false, 0);

    let pos_y_entry = Entry::new();
    pos_y_entry.set_placeholder_text(Some("Y Position"));
    cube_section.pack_start(&pos_y_entry, false, false, 0);

    let pos_z_entry = Entry::new();
    pos_z_entry.set_placeholder_text(Some("Z Position"));
    cube_section.pack_start(&pos_z_entry, false, false, 0);

    let radius_entry = Entry::new();
    radius_entry.set_placeholder_text(Some("Radius"));
    cube_section.pack_start(&radius_entry, false, false, 0);

    // Apply styles to entries
    let entries = vec![&pos_x_entry, &pos_y_entry, &pos_z_entry, &radius_entry];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    cube_section.pack_start(&material_selector, false, false, 0);

    // Apply styles to ComboBoxText
    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    let cube_config = CubeConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
    };
    app_state.borrow_mut().cubes.push(cube_config);

    cube_section
}

fn create_flat_plane_section(
    app_state: Rc<RefCell<AppState>>,
    flat_plane_count: usize,
) -> gtk::Box {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let flat_plane_section = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label_text = format!("Flat Plane {}", flat_plane_count);
    let flat_plane_label = gtk::Label::new(Some(&label_text));
    flat_plane_section.pack_start(&flat_plane_label, false, false, 0);
    let style_context = flat_plane_label.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

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

    // Apply styles to entries
    let entries = vec![&pos_x_entry, &pos_y_entry, &pos_z_entry, &radius_entry];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    flat_plane_section.pack_start(&material_selector, false, false, 0);

    // Apply styles to ComboBoxText
    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    let flat_plane_config = FlatPlaneConfig {
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
    };
    app_state.borrow_mut().flat_plane.push(flat_plane_config);

    flat_plane_section
}
