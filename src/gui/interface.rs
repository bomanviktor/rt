// use gdk::RGBA;
use gdk_pixbuf::Pixbuf;
use glib::clone;
use glib::signal::Inhibit;
use gtk::{prelude::*, Image};
use gtk::{
    Box as GtkBox, Button, ComboBoxText, CssProvider, Entry, Orientation, Scale, Separator, Window,
    WindowType,
};
use nalgebra::Vector3;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

use crate::color::Color;
use crate::config::Point;
use crate::objects::{Cube, Cylinder, FlatPlane, Objects, Sphere};
use crate::raytracer::{CameraBuilder, Scene};

pub struct AppState {
    pub spheres: Vec<SphereConfig>,
    pub cylinders: Vec<CylinderConfig>,
    pub cubes: Vec<CubeConfig>,
    pub flat_planes: Vec<FlatPlaneConfig>,
}
#[allow(dead_code)]
pub struct ObjectConfig {
    brightness_entry: Rc<RefCell<Entry>>,
    anti_aliasing_2x: Rc<RefCell<Entry>>,
    anti_aliasing_4x: Rc<RefCell<Entry>>,
    cam_x_entry: Rc<RefCell<Entry>>,
    cam_y_entry: Rc<RefCell<Entry>>,
    cam_angle_entry: Rc<RefCell<Entry>>,
    width_entry: Rc<RefCell<Entry>>,
    height_entry: Rc<RefCell<Entry>>,
}

pub struct SphereConfig {
    id: Rc<RefCell<u32>>,
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
    color_button: Rc<RefCell<gtk::ColorButton>>,
}
#[derive(Clone)]
pub struct CylinderConfig {
    id: Rc<RefCell<u32>>,
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
    height_entry: Rc<RefCell<Entry>>,
    color_button: Rc<RefCell<gtk::ColorButton>>,
}

pub struct CubeConfig {
    id: Rc<RefCell<u32>>,
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
    color_button: Rc<RefCell<gtk::ColorButton>>,
}

pub struct FlatPlaneConfig {
    id: Rc<RefCell<u32>>,
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    material_selector: Rc<RefCell<ComboBoxText>>,
    color_button: Rc<RefCell<gtk::ColorButton>>,
}

pub fn launch_gui(_app_state: Rc<RefCell<AppState>>) {
    let app_state = Rc::new(RefCell::new(AppState {
        spheres: Vec::new(),
        cylinders: Vec::new(),
        cubes: Vec::new(),
        flat_planes: Vec::new(),
    }));

    gtk::init().expect("Failed to initialize GTK.");

    // Create a CSS provider
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let window = Window::new(WindowType::Toplevel);
    window.set_resizable(true); // Allows the window to be resized
    window.set_decorated(true); // Allows the window to be decorated
    window.set_title("Ray Tracing Settings");
    window.set_default_size(800, 1200);
    window
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    let icon = Pixbuf::from_file("src/gui/RT.png").expect("Failed to load icon");
    let scaled_icon = icon
        .scale_simple(64, 64, gdk_pixbuf::InterpType::Bilinear)
        .expect("Failed to scale icon");
    window.set_icon(Some(&scaled_icon));

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    match Pixbuf::from_file("src/gui/RT.png") {
        Ok(icon) => window.set_icon(Some(&icon)),
        Err(err) => eprintln!("Failed to load icon: {}", err),
    }

    // About Dialog window
    let about_dialog = gtk::AboutDialog::new();
    about_dialog.set_program_name("Grit:Lab Ray Tracing Project");
    about_dialog.set_comments(Some(
        "Completed during grit:lab full-stack development course as part of the RUST Studies. \n
        December 2023",
    ));
    about_dialog.set_authors(&[
        "Viktor Boman",
        "Johannes Eckerman",
        "Salam Foon",
        "Ville Patjas",
        "André Teetor",
    ]);
    about_dialog.set_website_label(Some("Code Repository"));
    about_dialog.set_website(Some("https://github.com/bomanviktor/rt"));
    about_dialog.set_logo(Some(&scaled_icon));
    about_dialog.set_transient_for(Some(&window));
    about_dialog.set_modal(true);
    about_dialog.set_destroy_with_parent(true);

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    let top_vbox = GtkBox::new(Orientation::Horizontal, 10);
    vbox.set_border_width(10);
    vbox.set_spacing(10);

    let about_button = gtk::Button::with_label("About");
    about_button.connect_clicked(clone!(@weak about_dialog => move |_| {
        about_dialog.run();
        about_dialog.hide();
    }));
    top_vbox.pack_start(&about_button, false, false, 0);
    about_button.get_style_context().add_class("about-button");
    about_button
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    vbox.pack_start(&top_vbox, false, false, 0);
    scrolled_window.add(&vbox);
    window.add(&scrolled_window);

    let render_button = Button::with_label("Render picture");
    vbox.pack_start(&render_button, false, false, 0);
    render_button
        .get_style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Define CSS styles for the message label
    let red_style =
        "<span foreground='red'>Invalid input detected. Please enter numbers in 0.0 format.</span>";
    let green_style =
        "<span foreground='green'>All inputs are valid. Proceeding with rendering.</span>";

    // Create a label for displaying messages
    let message_label = gtk::Label::new(None);
    message_label.set_text("Ready"); // Default text
    vbox.pack_start(&message_label, false, false, 10); // Adjust packing as needed

    let show_image_button = Button::with_label("Show Image");
    vbox.pack_start(&show_image_button, false, false, 0);

    // Create a horizontal box for the side-by-side buttons
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_halign(gtk::Align::Center);

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

    // two checkboxes for choosing if the user wants 2x or 4x antialiasing
    let antialiasing_label = gtk::Label::new(Some("Antialiasing"));
    antialiasing_label
        .get_style_context()
        .add_class("antialiasing-label");
    vbox.pack_start(&antialiasing_label, false, false, 0);

    let antialiasing_2x = gtk::CheckButton::with_label("2x");
    antialiasing_2x
        .get_style_context()
        .add_class("antialiasing-check");
    antialiasing_2x.set_halign(gtk::Align::Center);
    vbox.pack_start(&antialiasing_2x, false, false, 0);

    let antialiasing_4x = gtk::CheckButton::with_label("4x");
    antialiasing_4x
        .get_style_context()
        .add_class("antialiasing-check");
    antialiasing_4x.set_halign(gtk::Align::Center);
    vbox.pack_start(&antialiasing_4x, false, false, 0);

    //if one of the checkboxes is clicked, the other one is unchecked
    antialiasing_2x.connect_clicked(clone!(@strong antialiasing_4x => move |_| {
        antialiasing_4x.set_active(false);
    }));

    antialiasing_4x.connect_clicked(clone!(@strong antialiasing_2x => move |_| {
        antialiasing_2x.set_active(false);
    }));

    // Add the horizontal box to the vertical box
    vbox.pack_start(&hbox, false, false, 0);

    let flow_box = gtk::FlowBox::new();
    flow_box.set_valign(gtk::Align::Start);
    flow_box.set_max_children_per_line(10); // Adjust as needed
    flow_box.set_selection_mode(gtk::SelectionMode::None);

    add_sphere_button.connect_clicked(clone!(@strong flow_box, @strong app_state => move |_| {
        create_sphere_section(app_state.clone(), flow_box.clone());
    }));

    add_cylinder_button.connect_clicked(clone!(@strong flow_box, @strong app_state => move |_| {
        create_cylinder_section(app_state.clone(), flow_box.clone());
    }));

    add_cube_button.connect_clicked(clone!(@strong flow_box, @strong app_state => move |_| {
        create_cube_section(app_state.clone(), flow_box.clone());
    }));

    add_flat_plane_button.connect_clicked(clone!(@strong flow_box, @strong app_state => move |_| {
        create_flat_plane_section(app_state.clone(), flow_box.clone());
    }));

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    vbox.pack_start(&separator, false, false, 10);

    // Brightness
    let adjustment = gtk::Adjustment::new(
        0.5, // initial value
        0.0, // minimum value
        1.0, // maximum value
        0.1, // step increment
        0.1, // page increment
        0.0, // page size
    );
    let brightness_label = gtk::Label::new(Some("Brightness"));
    vbox.pack_start(&brightness_label, false, false, 0);

    let brightness_entry = Scale::new(Orientation::Horizontal, Some(&adjustment));
    brightness_entry.set_value(0.5); // Set a default value
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

    let resolution_hbox = gtk::Box::new(Orientation::Horizontal, 5);
    resolution_hbox.set_halign(gtk::Align::Center);

    let width_entry = Entry::new();
    width_entry.set_placeholder_text(Some("Width"));
    resolution_hbox.pack_start(&width_entry, false, false, 0);

    let resolution_separator = gtk::Label::new(Some("x"));
    resolution_hbox.pack_start(&resolution_separator, false, false, 0);

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("Height"));
    resolution_hbox.pack_start(&height_entry, false, false, 0);
    vbox.pack_start(&resolution_hbox, false, false, 0);
    vbox.pack_start(&flow_box, false, false, 0);

    let brightness_entry_clone = brightness_entry.clone();
    let cam_x_entry_clone = cam_x_entry.clone();
    let cam_y_entry_clone = cam_y_entry.clone();
    let cam_angle_entry_clone = cam_angle_entry.clone();
    let width_entry_clone = width_entry.clone();
    let height_entry_clone = height_entry.clone();

    show_image_button.connect_clicked(move |_| {
        let image_window = Window::new(WindowType::Toplevel);
        image_window.set_title("Rendered Image");
        image_window.set_default_size(400, 400); // Set to your desired size

        let image = Image::from_file("output.ppm"); // Load the image
        image_window.add(&image);

        image_window.show_all();
    });

    // Render Button
    render_button.connect_clicked(clone!(@strong app_state, @strong message_label => move |_| {
        let mut all_inputs_valid = true;
        let app_state_borrowed = app_state.borrow();

    // Iterate and validate sphere inputs
    for (index, sphere) in app_state_borrowed.spheres.iter().enumerate() {
        let pos_x = sphere.pos_x_entry.borrow().get_text().to_string();
        let pos_y = sphere.pos_y_entry.borrow().get_text().to_string();
        let pos_z = sphere.pos_z_entry.borrow().get_text().to_string();
        let radius = sphere.radius_entry.borrow().get_text().to_string();

        // Check if inputs are valid floats
        if !is_valid_float(&pos_x) || !is_valid_float(&pos_y) || !is_valid_float(&pos_z) || !is_valid_float(&radius) {
            all_inputs_valid = false;
            println!("Invalid input for Sphere {}: X: {}, Y: {}, Z: {}, Radius: {}", index + 1, pos_x, pos_y, pos_z, radius);
            break; // Stop checking further if any invalid input is found
        }

        // Retrieve and print other properties
        let material = sphere.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());
        let sphere_color = sphere.color_button.borrow().get_rgba();
        let (r, g, b) = (sphere_color.red * 255.0, sphere_color.green * 255.0, sphere_color.blue * 255.0);
        println!("Valid Sphere {}: X: {}, Y: {}, Z: {}, Radius: {}, Material: {}, Color: RGB({}, {}, {})", 
                 index + 1, pos_x, pos_y, pos_z, radius, material, r as u8, g as u8, b as u8);
    }


    for (index, cylinder) in app_state_borrowed.cylinders.iter().enumerate() {
        let pos_x = cylinder.pos_x_entry.borrow().get_text().to_string();
        let pos_y = cylinder.pos_y_entry.borrow().get_text().to_string();
        let pos_z = cylinder.pos_z_entry.borrow().get_text().to_string();
        let radius = cylinder.radius_entry.borrow().get_text().to_string();
        let height = cylinder.height_entry.borrow().get_text().to_string();

        if !is_valid_float(&pos_x) || !is_valid_float(&pos_y) || !is_valid_float(&pos_z) ||
           !is_valid_float(&radius) || !is_valid_float(&height) {
            all_inputs_valid = false;
            println!("Invalid input for Cylinder {}: X: {}, Y: {}, Z: {}, Radius: {}, Height: {}", index + 1, pos_x, pos_y, pos_z, radius, height);
            break;
        }

        let material = cylinder.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());
        let cylinder_color = cylinder.color_button.borrow().get_rgba();
        let (r, g, b) = (cylinder_color.red * 255.0, cylinder_color.green * 255.0, cylinder_color.blue * 255.0);

        println!("Cylinder {}: X: {}, Y: {}, Z: {}, Radius: {}, Height: {}, Material: {}, Color: RGB({}, {}, {})", 
                 index + 1, pos_x, pos_y, pos_z, radius, height, material, r as u8, g as u8, b as u8);
    }


        for (index, cube) in app_state_borrowed.cubes.iter().enumerate() {
            let pos_x = cube.pos_x_entry.borrow().get_text().to_string();
            let pos_y = cube.pos_y_entry.borrow().get_text().to_string();
            let pos_z = cube.pos_z_entry.borrow().get_text().to_string();
            let radius = cube.radius_entry.borrow().get_text().to_string();

        // Check if inputs are valid floats
        if !is_valid_float(&pos_x) || !is_valid_float(&pos_y) || !is_valid_float(&pos_z) || !is_valid_float(&radius) {
            all_inputs_valid = false;
            println!("Invalid input for Cube {}: X: {}, Y: {}, Z: {}, Radius: {}", index + 1, pos_x, pos_y, pos_z, radius);
            break; // Stop checking further if any invalid input is found
        }
            let material = cube.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());
            let cube_color = cube.color_button.borrow().get_rgba();
            let (r, g, b) = (cube_color.red * 255.0, cube_color.green * 255.0, cube_color.blue * 255.0);

            println!("Valid Cube{}: X: {}, Y: {}, Z: {}, Radius: {}, Material: {}, Color: RGB({}, {}, {})", 
            index + 1, pos_x, pos_y, pos_z, radius, material, r as u8, g as u8, b as u8);
        }

        for (index, flat_plane) in app_state_borrowed.flat_planes.iter().enumerate() {
            let pos_x = flat_plane.pos_x_entry.borrow().get_text().to_string();
            let pos_y = flat_plane.pos_y_entry.borrow().get_text().to_string();
            let pos_z = flat_plane.pos_z_entry.borrow().get_text().to_string();
            let radius = flat_plane.radius_entry.borrow().get_text().to_string();

        // Check if inputs are valid floats
        if !is_valid_float(&pos_x) || !is_valid_float(&pos_y) || !is_valid_float(&pos_z) || !is_valid_float(&radius) {
            all_inputs_valid = false;
            println!("Invalid input for Flat plane {}: X: {}, Y: {}, Z: {}, Radius: {}", index + 1, pos_x, pos_y, pos_z, radius);
            break; // Stop checking further if any invalid input is found
        }

            let material = flat_plane.material_selector.borrow().get_active_text().unwrap_or_else(|| "Lambertian".into());
            let flat_plane_color = flat_plane.color_button.borrow().get_rgba();
            let (r, g, b) = (flat_plane_color.red * 255.0, flat_plane_color.green * 255.0, flat_plane_color.blue * 255.0);

            println!("Valid Flat Plane {}: X: {}, Y: {}, Z: {}, Radius: {}, Material: {}, Color: RGB({}, {}, {})", 
            index + 1, pos_x, pos_y, pos_z, radius, material, r as u8, g as u8, b as u8);
        }
    // Check antialiasing options
    let antialiasing = if antialiasing_2x.get_active() {
        "2x"
    } else if antialiasing_4x.get_active() {
        "4x"
    } else {
        "None"
    };


        println!("Brightness: {}", brightness_entry_clone.get_value());
        println!("Antialiasing option selected: {}", antialiasing);
        println!("Camera X Position: {}", cam_x_entry_clone.get_text());
        println!("Camera Y Position: {}", cam_y_entry_clone.get_text());
        println!("Camera Angle: {}", cam_angle_entry_clone.get_text());
        println!("Resolution Width: {}", width_entry_clone.get_text());
        println!("Resolution Height: {}", height_entry_clone.get_text());

        let mut cam_x = 0.0;
        let mut cam_y = 0.0;
        let mut cam_angle = 0.0;
        let mut width = 0;
        let mut height = 0;
        // let mut Brightness = 0.0;

        if let (Ok(x), Ok(y), Ok(angle), Ok(w), Ok(h)) = (
            cam_x_entry_clone.get_text().parse::<f64>(),
            cam_y_entry_clone.get_text().parse::<f64>(),
            cam_angle_entry_clone.get_text().parse::<f64>(),
            width_entry_clone.get_text().parse::<u32>(),
            height_entry_clone.get_text().parse::<u32>(),
            // let mut Brightness = brightness_entry_clone.get_value(),
        ) {
            cam_x = x;
            cam_y = y;
            cam_angle = angle;
            width = w;
            height = h;
            // brightness = Brightness;
        } else {
            all_inputs_valid = false;
        }

        if all_inputs_valid {
            println!("All inputs are valid. Proceeding with rendering.");
            message_label.set_markup(green_style);

            // Schedule rendering to start after a short delay
            glib::timeout_add_local(50, clone!(@strong app_state => move || {
                const OUTPUT_PATH: &str = "output.ppm";
                let updated_scene = update_scene_from_gui(app_state.clone());
                let mut camera = CameraBuilder::new()
                .sample_size(1)
                .position_by_coordinates(Vector3::new(cam_x, cam_y, cam_angle))
                .look_at(Vector3::new(0.0, 0.0, 0.0))
                .up_direction_by_coordinates(Vector3::new(0.0, 1.0, 0.0))
                .focal_length(0.5)
                .resolution((width, height))
                .sensor_width(1.0)
                .build();

                camera.send_rays(&updated_scene.objects);
                camera.write_to_ppm(OUTPUT_PATH);

                glib::Continue(false)
            }));
        } else {
            println!("Invalid input detected. Please enter numbers in 0.0 format");
            message_label.set_markup(red_style);
        }
    }));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

// Function to validate position entries
fn is_valid_float(input: &str) -> bool {
    // Check if the input is a valid floating-point number
    let is_float = input.parse::<f64>().is_ok();

    // Check if the input contains a decimal point
    let has_decimal_point = input.contains('.');

    // The input is valid if it's a float and contains a decimal point
    is_float && has_decimal_point
}

fn create_sphere_section(app_state: Rc<RefCell<AppState>>, flow_box: gtk::FlowBox) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let sphere_count = app_state.borrow().spheres.len();
    let unique_id = format!("sphere_{}", sphere_count); // Generate unique ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid
    print!("Setting grid ID: '{}'", grid.get_widget_name()); // Debug print for grid ID
    println!("Grid set with widget name: {}", unique_id); // Debug print for grid ID

    let label_text = format!("Sphere {}:", sphere_count);
    let sphere_label = gtk::Label::new(Some(&label_text));
    grid.attach(&sphere_label, 0, 0, 1, 1); // Column 0, Row 0 (Sphere label)

    let pos_x_label = gtk::Label::new(Some("X Position"));
    grid.attach(&pos_x_label, 0, 1, 1, 1); // Column 0, Row 1 (X Position label)

    let pos_x_entry = Entry::new();
    pos_x_entry.set_text("0.0"); // Set default text
    grid.attach(&pos_x_entry, 0, 2, 1, 1); // Column 0, Row 2 (X Position entry)

    let pos_y_label = gtk::Label::new(Some("Y Position"));
    grid.attach(&pos_y_label, 0, 3, 1, 1); // Column 0, Row 3

    let pos_y_entry = Entry::new();
    pos_y_entry.set_text("0.0");
    grid.attach(&pos_y_entry, 0, 4, 1, 1); // Column 0, Row 4

    let pos_z_label = gtk::Label::new(Some("Z Position"));
    grid.attach(&pos_z_label, 0, 5, 1, 1); // Column 0, Row 5

    let pos_z_entry = Entry::new();
    pos_z_entry.set_text("0.0");
    grid.attach(&pos_z_entry, 0, 6, 1, 1); // Column 0, Row 6

    // Radius Label and Entry
    let radius_label = gtk::Label::new(Some("Radius"));
    grid.attach(&radius_label, 0, 7, 1, 1); // Column 0, Row 7

    let radius_entry = Entry::new();
    radius_entry.set_text("0.0"); // Set default text
    grid.attach(&radius_entry, 0, 8, 1, 1); // Column 0, Row 8

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Material"));
    grid.attach(&material_label, 0, 9, 1, 1); // Column 0, Row 9

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    grid.attach(&material_selector, 0, 10, 1, 1); // Column 0, Row 10

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 11, 1, 1); // Column 0, Row 11

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 12, 1, 1); // Column 0, Row 12

    // Apply styles to ComboBoxText and Entries
    let entries = vec![&pos_x_entry, &pos_y_entry, &pos_z_entry, &radius_entry];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    let sphere_config = SphereConfig {
        id: Rc::new(RefCell::new(sphere_count as u32)),
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry.clone())),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry.clone())),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry.clone())),
        radius_entry: Rc::new(RefCell::new(radius_entry.clone())),
        material_selector: Rc::new(RefCell::new(material_selector.clone())),
        color_button: Rc::new(RefCell::new(color_button.clone())),
    };

    // Create a randomize button for the sphere section
    let randomize_button = gtk::Button::with_label("Randomize");
    grid.attach(&randomize_button, 0, 13, 1, 1); // Adjust the row number as needed

    // Connect the randomize button click handler
    let pos_x_entry_clone = pos_x_entry.clone();
    let pos_y_entry_clone = pos_y_entry.clone();
    let pos_z_entry_clone = pos_z_entry.clone();
    let radius_entry_clone = radius_entry.clone();
    let material_selector_clone = material_selector.clone();
    randomize_button.connect_clicked(move |_| {
        let mut rng = rand::thread_rng();

        pos_x_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        pos_y_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        pos_z_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        radius_entry_clone.set_text(&format!("{:.2}", rng.gen_range(0.1..5.0)));

        // Randomly select a material
        let materials = ["Lambertian", "Metal", "Dielectric"];
        let random_material_index = rng.gen_range(0..materials.len());
        material_selector_clone.set_active(Some(random_material_index as u32));
    });

    // Create a delete button for the sphere section
    let delete_id = sphere_config.id.clone();
    let delete_button = Button::with_label("Delete");
    println!("Adding delete button with ID: {}", *delete_id.borrow());
    grid.attach(&delete_button, 0, 14, 1, 1); // Column 0, Row 13

    // Connect the delete button click handler
    delete_button.connect_clicked(clone!(@strong app_state, @strong flow_box => move |_| {
    let id_number = *delete_id.borrow();
    let id = format!("sphere_{}", id_number);
    println!("Attempting to delete sphere with ID: {}", id);

    // Debug: Print current sphere IDs before deletion
    println!("Current sphere IDs before deletion:");
    for sphere in app_state.borrow().spheres.iter() {
        println!("Sphere ID: {}", *sphere.id.borrow());
    }
    #[allow(unused_assignments)]
    let mut deletion_successful = false;
    {
        let mut app_state = app_state.borrow_mut();
        if let Some(index) = app_state.spheres.iter().position(|s| format!("sphere_{}", *s.id.borrow()) == id) {
            app_state.spheres.remove(index);
            deletion_successful = true;
        } else {
            eprintln!("Error: No sphere with ID {} found in app_state", id);
            return;
        }
    }

    // Debug: Inspect the children of flow_box before attempting deletion
    println!("Inspecting GUI elements in flow_box before deletion:");
    let children = flow_box.get_children();
    for (index, child) in children.iter().enumerate() {
        // Attempt to downcast the child to GtkFlowBoxChild
        if let Some(flowbox_child) = child.downcast_ref::<gtk::FlowBoxChild>() {
            if let Some(widget) = flowbox_child.get_child() {
                let widget_name = widget.get_widget_name().to_string(); // Get the name of the widget inside the GtkFlowBoxChild
                println!("Child {}: GUI element ID inside GtkFlowBoxChild: {}", index, widget_name);
                println!("Child {}: Type: {}", index, widget);

                if widget_name == id {
                    flow_box.remove(child);
                    deletion_successful = true;
                    break;
                }
            }
        }
    }

    if deletion_successful {
        println!("Successfully deleted sphere with ID: {}", id);
    } else {
        eprintln!("Error: GUI element for sphere with ID {} not found", id);
    }

    // Borrow app_state again for reading
    println!("Current sphere IDs after deletion:");
    for sphere in app_state.borrow().spheres.iter() {
        println!("Sphere ID: {}", *sphere.id.borrow());
    }

    flow_box.show_all();
}));

    app_state.borrow_mut().spheres.push(sphere_config);

    flow_box.add(&grid); // Directly add the grid to the flow_box
    flow_box.show_all();

    // Debug: Print the ID of the created GUI element
    println!("Added GUI element with ID: {}", unique_id);

    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

fn create_cylinder_section(
    app_state: Rc<RefCell<AppState>>,
    flow_box: gtk::FlowBox,
) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let cylinder_count = app_state.borrow().cylinders.len();
    let unique_id = format!("cylinder_{}", cylinder_count); // Generate unique ID
    println!("Creating cylinder section with ID: {}", unique_id); // Debug print for cylinder ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid

    let label_text = format!("Cylinder {}:", cylinder_count);
    let cylinder_label = gtk::Label::new(Some(&label_text));
    grid.attach(&cylinder_label, 0, 0, 1, 1); // Column 0, Row 0

    let pos_x_label = gtk::Label::new(Some("X Position"));
    grid.attach(&pos_x_label, 0, 1, 1, 1); // Column 0, Row 1 (X Position label)

    let pos_x_entry = Entry::new();
    pos_x_entry.set_text("0.0"); // Set default text
    grid.attach(&pos_x_entry, 0, 2, 1, 1); // Column 0, Row 2 (X Position entry)

    let pos_y_label = gtk::Label::new(Some("Y Position"));
    grid.attach(&pos_y_label, 0, 3, 1, 1); // Column 0, Row 3

    let pos_y_entry = Entry::new();
    pos_y_entry.set_text("0.0");
    grid.attach(&pos_y_entry, 0, 4, 1, 1); // Column 0, Row 4

    let pos_z_label = gtk::Label::new(Some("Z Position"));
    grid.attach(&pos_z_label, 0, 5, 1, 1); // Column 0, Row 5

    let pos_z_entry = Entry::new();
    pos_z_entry.set_text("0.0");
    grid.attach(&pos_z_entry, 0, 6, 1, 1); // Column 0, Row 6

    // Radius Label and Entry
    let radius_label = gtk::Label::new(Some("Radius"));
    grid.attach(&radius_label, 0, 7, 1, 1); // Column 0, Row 7

    let radius_entry = Entry::new();
    radius_entry.set_text("0.0"); // Set default text
    grid.attach(&radius_entry, 0, 8, 1, 1); // Column 0, Row 8

    let height_label = gtk::Label::new(Some("Height"));
    grid.attach(&height_label, 0, 9, 1, 1); // Column 0, Row 9
    let height_entry = Entry::new();
    height_entry.set_text("0.0"); // Set default text
    height_entry.set_placeholder_text(Some("Height"));
    grid.attach(&height_entry, 0, 10, 1, 1); // Column 0, Row 10

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

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Material"));
    grid.attach(&material_label, 0, 11, 1, 1); // Column 0, Row 11

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    grid.attach(&material_selector, 0, 12, 1, 1); // Column 0, Row 12

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 13, 1, 1); // Column 0, Row 13

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 14, 1, 1); // Column 0, Row 14

    // Apply styles to ComboBoxText and Entries
    let entries = vec![&pos_x_entry, &pos_y_entry, &pos_z_entry, &radius_entry];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    let cylinder_config = CylinderConfig {
        id: Rc::new(RefCell::new(cylinder_count as u32)),
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry.clone())),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry.clone())),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry.clone())),
        radius_entry: Rc::new(RefCell::new(radius_entry.clone())),
        height_entry: Rc::new(RefCell::new(height_entry.clone())),
        material_selector: Rc::new(RefCell::new(material_selector.clone())),
        color_button: Rc::new(RefCell::new(color_button.clone())),
    };

    let randomize_button = gtk::Button::with_label("Randomize");
    grid.attach(&randomize_button, 0, 15, 1, 1); // Adjust the row number as needed

    // Connect the randomize button click handler
    let pos_x_entry_clone = pos_x_entry.clone();
    let pos_y_entry_clone = pos_y_entry.clone();
    let pos_z_entry_clone = pos_z_entry.clone();
    let radius_entry_clone = radius_entry.clone();
    let height_entry_clone = height_entry.clone();
    let material_selector_clone = material_selector.clone();
    randomize_button.connect_clicked(move |_| {
        let mut rng = rand::thread_rng();

        pos_x_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        pos_y_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        pos_z_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        radius_entry_clone.set_text(&format!("{:.2}", rng.gen_range(0.1..5.0)));
        height_entry_clone.set_text(&format!("{:.2}", rng.gen_range(0.1..5.0)));

        // Randomly select a material
        let materials = ["Lambertian", "Metal", "Dielectric"];
        let random_material_index = rng.gen_range(0..materials.len());
        material_selector_clone.set_active(Some(random_material_index as u32));
    });

    // Create a delete button for the cylinder section
    let delete_id = cylinder_config.id.clone();
    let delete_button = gtk::Button::with_label("Delete");
    grid.attach(&delete_button, 0, 16, 1, 1); // Column 0, Row 15

    // Connect the delete button click handler
    delete_button.connect_clicked(clone!(@strong app_state, @strong flow_box => move |_| {
        let id_number = *delete_id.borrow();
        let id = format!("cylinder_{}", id_number);
        println!("Attempting to delete cylinder with ID: {}", id);

        // Debug: Print current cylinder IDs before deletion
        println!("Current cylinder IDs before deletion:");
        for cylinder in app_state.borrow().cylinders.iter() {
            println!("cylinder ID: {}", *cylinder.id.borrow());
        }
        #[allow(unused_assignments)]
        let mut deletion_successful = false;
        {
            let mut app_state = app_state.borrow_mut();
            if let Some(index) = app_state.cylinders.iter().position(|s| format!("cylinder_{}", *s.id.borrow()) == id) {
                app_state.cylinders.remove(index);
                deletion_successful = true;
            } else {
                eprintln!("Error: No cylinder with ID {} found in app_state", id);
                return;
            }
        }

        // Debug: Inspect the children of flow_box before attempting deletion
        println!("Inspecting GUI elements in flow_box before deletion:");
        let children = flow_box.get_children();
        for (index, child) in children.iter().enumerate() {
            // Attempt to downcast the child to GtkFlowBoxChild
            if let Some(flowbox_child) = child.downcast_ref::<gtk::FlowBoxChild>() {
                if let Some(widget) = flowbox_child.get_child() {
                    let widget_name = widget.get_widget_name().to_string(); // Get the name of the widget inside the GtkFlowBoxChild
                    println!("Child {}: GUI element ID inside GtkFlowBoxChild: {}", index, widget_name);
                    println!("Child {}: Type: {}", index, widget);

                    if widget_name == id {
                        flow_box.remove(child);
                        deletion_successful = true;
                        break;
                    }
                }
            }
        }

        if deletion_successful {
            println!("Successfully deleted cylinder with ID: {}", id);
        } else {
            eprintln!("Error: GUI element for cylinder with ID {} not found", id);
        }

        // Borrow app_state again for reading
        println!("Current cylinder IDs after deletion:");
        for cylinder in app_state.borrow().cylinders.iter() {
            println!("cylinder ID: {}", *cylinder.id.borrow());
        }

        flow_box.show_all();
    }));

    // Add the configuration to the AppState
    app_state.borrow_mut().cylinders.push(cylinder_config);

    flow_box.add(&grid);
    flow_box.show_all();
    print!("Added GUI element with ID: '{}'", unique_id); // Debug print for cylinder ID
    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

//Todo: refine this function
fn create_cube_section(app_state: Rc<RefCell<AppState>>, flow_box: gtk::FlowBox) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let cube_count = app_state.borrow().cubes.len();
    let unique_id = format!("cube_{}", cube_count); // Generate unique ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid

    let label_text = format!("Cube {}:", cube_count);
    let cube_label = gtk::Label::new(Some(&label_text));
    grid.attach(&cube_label, 0, 0, 1, 1); // Column 0, Row 0

    let pos_x_label = gtk::Label::new(Some("X Position"));
    grid.attach(&pos_x_label, 0, 1, 1, 1); // Column 0, Row 1 (X Position label)

    let pos_x_entry = Entry::new();
    pos_x_entry.set_text("0.0"); // Set default text
    grid.attach(&pos_x_entry, 0, 2, 1, 1); // Column 0, Row 2 (X Position entry)

    let pos_y_label = gtk::Label::new(Some("Y Position"));
    grid.attach(&pos_y_label, 0, 3, 1, 1); // Column 0, Row 3

    let pos_y_entry = Entry::new();
    pos_y_entry.set_text("0.0");
    grid.attach(&pos_y_entry, 0, 4, 1, 1); // Column 0, Row 4

    let pos_z_label = gtk::Label::new(Some("Z Position"));
    grid.attach(&pos_z_label, 0, 5, 1, 1); // Column 0, Row 5

    let pos_z_entry = Entry::new();
    pos_z_entry.set_text("0.0");
    grid.attach(&pos_z_entry, 0, 6, 1, 1); // Column 0, Row 6

    // Radius Label and Entry
    let radius_label = gtk::Label::new(Some("Radius"));
    grid.attach(&radius_label, 0, 7, 1, 1); // Column 0, Row 7

    let radius_entry = Entry::new();
    radius_entry.set_text("0.0"); // Set default text
    grid.attach(&radius_entry, 0, 8, 1, 1); // Column 0, Row 8

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Material"));
    grid.attach(&material_label, 0, 9, 1, 1); // Column 0, Row 9

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    grid.attach(&material_selector, 0, 10, 1, 1); // Column 0, Row 10

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 11, 1, 1); // Column 0, Row 11

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 12, 1, 1); // Column 0, Row 12

    // Apply styles to ComboBoxText and Entries
    let entries = vec![&pos_x_entry, &pos_y_entry, &pos_z_entry, &radius_entry];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    grid.set_widget_name(&unique_id);

    let cube_config = CubeConfig {
        id: Rc::new(RefCell::new(cube_count as u32)),
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry.clone())),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry.clone())),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry.clone())),
        radius_entry: Rc::new(RefCell::new(radius_entry.clone())),
        material_selector: Rc::new(RefCell::new(material_selector.clone())),
        color_button: Rc::new(RefCell::new(color_button.clone())),
    };

    let randomize_button = gtk::Button::with_label("Randomize");
    grid.attach(&randomize_button, 0, 13, 1, 1); // Adjust the row number as needed

    // Connect the randomize button click handler
    let pos_x_entry_clone = pos_x_entry.clone();
    let pos_y_entry_clone = pos_y_entry.clone();
    let pos_z_entry_clone = pos_z_entry.clone();
    let radius_entry_clone = radius_entry.clone();
    let material_selector_clone = material_selector.clone();
    randomize_button.connect_clicked(move |_| {
        let mut rng = rand::thread_rng();

        pos_x_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        pos_y_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        pos_z_entry_clone.set_text(&format!("{:.2}", rng.gen_range(-10.0..10.0)));
        radius_entry_clone.set_text(&format!("{:.2}", rng.gen_range(0.1..5.0)));

        // Randomly select a material
        let materials = ["Lambertian", "Metal", "Dielectric"];
        let random_material_index = rng.gen_range(0..materials.len());
        material_selector_clone.set_active(Some(random_material_index as u32));
    });
    let delete_id = cube_config.id.clone();
    let delete_button = gtk::Button::with_label("Delete");
    grid.attach(&delete_button, 0, 14, 1, 1); //Column 0, Row 13

    // Connect a handler to the delete button
    delete_button.connect_clicked(clone!(@strong app_state, @strong flow_box => move |_| {
        let id_number = *delete_id.borrow();
        let id = format!("cube_{}", id_number);
        println!("Attempting to delete cube with ID: {}", id);

        // Debug: Print current cube IDs before deletion
        println!("Current cube IDs before deletion:");
        for cube in app_state.borrow().cubes.iter() {
            println!("cube ID: {}", *cube.id.borrow());
        }
        #[allow(unused_assignments)]
        let mut deletion_successful = false;
        {
            let mut app_state = app_state.borrow_mut();
            if let Some(index) = app_state.cubes.iter().position(|s| format!("cube_{}", *s.id.borrow()) == id) {
                app_state.cubes.remove(index);
                deletion_successful = true;
            } else {
                eprintln!("Error: No cube with ID {} found in app_state", id);
                return;
            }
        }

        // Debug: Inspect the children of flow_box before attempting deletion
        println!("Inspecting GUI elements in flow_box before deletion:");
        let children = flow_box.get_children();
        for (index, child) in children.iter().enumerate() {
            // Attempt to downcast the child to GtkFlowBoxChild
            if let Some(flowbox_child) = child.downcast_ref::<gtk::FlowBoxChild>() {
                if let Some(widget) = flowbox_child.get_child() {
                    let widget_name = widget.get_widget_name().to_string(); // Get the name of the widget inside the GtkFlowBoxChild
                    println!("Child {}: GUI element ID inside GtkFlowBoxChild: {}", index, widget_name);
                    println!("Child {}: Type: {}", index, widget);

                    if widget_name == id {
                        flow_box.remove(child);
                        deletion_successful = true;
                        break;
                    }
                }
            }
        }

        if deletion_successful {
            println!("Successfully deleted cube with ID: {}", id);
        } else {
            eprintln!("Error: GUI element for cube with ID {} not found", id);
        }

        // Borrow app_state again for reading
        println!("Current cube IDs after deletion:");
        for cube in app_state.borrow().cubes.iter() {
            println!("cube ID: {}", *cube.id.borrow());
        }

        flow_box.show_all();
    }));

    app_state.borrow_mut().cubes.push(cube_config);

    flow_box.add(&grid); // Directly add the grid to the flow_box
    flow_box.show_all();
    print!("Added GUI element with ID: '{}'", unique_id); // Debug print for cube ID
    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

fn create_flat_plane_section(
    app_state: Rc<RefCell<AppState>>,
    flow_box: gtk::FlowBox,
) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let flat_plane_count = app_state.borrow().flat_planes.len();
    let unique_id = format!("flat_plane_{}", flat_plane_count); // Generate unique ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid

    let label_text = format!("Flat Plane {}:", flat_plane_count);
    let flat_plane_label = gtk::Label::new(Some(&label_text));
    grid.attach(&flat_plane_label, 0, 0, 1, 1); // Column 0, Row 0

    let pos_x_label = gtk::Label::new(Some("X Position"));
    grid.attach(&pos_x_label, 0, 1, 1, 1); // Column 0, Row 1 (X Position label)

    let pos_x_entry = Entry::new();
    pos_x_entry.set_text("0.0"); // Set default text
    grid.attach(&pos_x_entry, 0, 2, 1, 1); // Column 0, Row 2 (X Position entry)

    let pos_y_label = gtk::Label::new(Some("Y Position"));
    grid.attach(&pos_y_label, 0, 3, 1, 1); // Column 0, Row 3

    let pos_y_entry = Entry::new();
    pos_y_entry.set_text("0.0");
    grid.attach(&pos_y_entry, 0, 4, 1, 1); // Column 0, Row 4

    let pos_z_label = gtk::Label::new(Some("Z Position"));
    grid.attach(&pos_z_label, 0, 5, 1, 1); // Column 0, Row 5

    let pos_z_entry = Entry::new();
    pos_z_entry.set_text("0.0");
    grid.attach(&pos_z_entry, 0, 6, 1, 1); // Column 0, Row 6

    // Radius Label and Entry
    let radius_label = gtk::Label::new(Some("Radius"));
    grid.attach(&radius_label, 0, 7, 1, 1); // Column 0, Row 7

    let radius_entry = Entry::new();
    radius_entry.set_text("10.0"); // Set default text
    grid.attach(&radius_entry, 0, 8, 1, 1); // Column 0, Row 8

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Material"));
    grid.attach(&material_label, 0, 9, 1, 1); // Column 0, Row 9

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Lambertian");
    material_selector.append_text("Metal");
    material_selector.append_text("Dielectric");
    material_selector.set_active(Some(0));
    grid.attach(&material_selector, 0, 10, 1, 1); // Column 0, Row 10

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 11, 1, 1); // Column 0, Row 11

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 12, 1, 1); // Column 0, Row 12

    // Apply styles to ComboBoxText and Entries
    let entries = vec![&pos_x_entry, &pos_y_entry, &pos_z_entry, &radius_entry];
    for entry in entries {
        let style_context = entry.get_style_context();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    }

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    grid.set_widget_name(&unique_id);

    let flat_plane_config = FlatPlaneConfig {
        id: Rc::new(RefCell::new(flat_plane_count as u32)),
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry)),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry)),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry)),
        radius_entry: Rc::new(RefCell::new(radius_entry)),
        material_selector: Rc::new(RefCell::new(material_selector)),
        color_button: Rc::new(RefCell::new(color_button)),
    };

    let delete_id = flat_plane_config.id.clone();
    let delete_button = gtk::Button::with_label("Delete");
    grid.attach(&delete_button, 0, 13, 1, 1); //Column 0, Row 13

    // Connect a handler to the delete button
    delete_button.connect_clicked(clone!(@strong app_state, @strong flow_box => move |_| {
        let id_number = *delete_id.borrow();
        let id = format!("flat_plane_{}", id_number);
        println!("Attempting to delete flat_plane with ID: {}", id);
        // Debug: Print current flat_plane IDs before deletion
        println!("Current flat_plane IDs before deletion:");
        for flat_plane in app_state.borrow().flat_planes.iter() {
            println!("flat_plane ID: {}", *flat_plane.id.borrow());
        }
        #[allow(unused_assignments)]
        let mut deletion_successful = false;
        {
            let mut app_state = app_state.borrow_mut();
            if let Some(index) = app_state.flat_planes.iter().position(|s| format!("flat_plane_{}", *s.id.borrow()) == id) {
                app_state.flat_planes.remove(index);
                deletion_successful = true;
            } else {
                eprintln!("Error: No flat_plane with ID {} found in app_state", id);
                return;
            }
        }
        // Debug: Inspect the children of flow_box before attempting deletion
        println!("Inspecting GUI elements in flow_box before deletion:");
        let children = flow_box.get_children();
        for (index, child) in children.iter().enumerate() {
            // Attempt to downcast the child to GtkFlowBoxChild
            if let Some(flowbox_child) = child.downcast_ref::<gtk::FlowBoxChild>() {
                if let Some(widget) = flowbox_child.get_child() {
                    let widget_name = widget.get_widget_name().to_string(); // Get the name of the widget inside the GtkFlowBoxChild
                    println!("Child {}: GUI element ID inside GtkFlowBoxChild: {}", index, widget_name);
                    println!("Child {}: Type: {}", index, widget);

                    if widget_name == id {
                        flow_box.remove(child);
                        deletion_successful = true;
                        break;
                    }
                }
            }
        }
        if deletion_successful {
            println!("Successfully deleted flat_plane with ID: {}", id);
        } else {
            eprintln!("Error: GUI element for flat_plane with ID {} not found", id);
        }
        // Borrow app_state again for reading
        println!("Current flat_plane IDs after deletion:");
        for flat_plane in app_state.borrow().flat_planes.iter() {
            println!("Sphere ID: {}", *flat_plane.id.borrow());
        }
        flow_box.show_all();
    }));

    app_state.borrow_mut().flat_planes.push(flat_plane_config);

    flow_box.add(&grid); // Directly add the grid to the flow_box
    flow_box.show_all();
    print!("Added GUI element with ID: '{}'", unique_id); // Debug print for flat_plane ID

    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

pub fn update_scene_from_gui(app_state: Rc<RefCell<AppState>>) -> Scene {
    let app_state_borrowed = app_state.borrow();
    let mut objects: Objects = Vec::new();

    // Creating Spheres
    for sphere_config in app_state_borrowed.spheres.iter() {
        let pos_x = sphere_config
            .pos_x_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_y = sphere_config
            .pos_y_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_z = sphere_config
            .pos_z_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let radius = sphere_config
            .radius_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(1.0);
        let color = sphere_config.color_button.borrow().get_rgba();
        let sphere_color = Color::new(
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        );

        let sphere = Sphere::new(Vector3::new(pos_x, pos_y, pos_z), radius, sphere_color);
        objects.push(Box::new(sphere));
    }

    // Creating Cylinders
    for cylinder_config in app_state_borrowed.cylinders.iter() {
        let pos_x = cylinder_config
            .pos_x_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_y = cylinder_config
            .pos_y_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_z = cylinder_config
            .pos_z_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let radius = cylinder_config
            .radius_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(1.0);
        let height = cylinder_config
            .height_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(1.0);
        let color = cylinder_config.color_button.borrow().get_rgba();
        let cylinder_color = Color::new(
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        );

        let cylinder = Cylinder::new(
            Vector3::new(pos_x, pos_y, pos_z),
            radius,
            height,
            cylinder_color,
        );
        objects.push(Box::new(cylinder));
    }

    // Creating Cubes
    for cube_config in app_state_borrowed.cubes.iter() {
        let pos_x = cube_config
            .pos_x_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_y = cube_config
            .pos_y_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_z = cube_config
            .pos_z_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let radius = cube_config
            .radius_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(1.0);
        let color = cube_config.color_button.borrow().get_rgba();
        let cube_color = Color::new(
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        );

        let cube = Cube::new(Vector3::new(pos_x, pos_y, pos_z), radius, cube_color);
        objects.push(Box::new(cube));
    }

    // Creating Flat Planes
    for flat_plane_config in app_state_borrowed.flat_planes.iter() {
        let pos_x = flat_plane_config
            .pos_x_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_y = flat_plane_config
            .pos_y_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let pos_z = flat_plane_config
            .pos_z_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(0.0);
        let radius = flat_plane_config
            .radius_entry
            .borrow()
            .get_text()
            .parse::<f64>()
            .unwrap_or(1.0);
        let color = flat_plane_config.color_button.borrow().get_rgba();
        let flat_plane_color = Color::new(
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        );

        let flat_plane =
            FlatPlane::new(Vector3::new(pos_x, pos_y, pos_z), radius, flat_plane_color);
        objects.push(Box::new(flat_plane));
    }

    Scene {
        objects,
        origo: Point::default(),
    }
}
