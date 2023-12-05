use gdk_pixbuf::Pixbuf;
use glib::clone;
use glib::signal::Inhibit;
use gtk::{prelude::*, Image};
use gtk::{
    Box as GtkBox, Button, ComboBoxText, CssProvider, Entry, Orientation, Scale, Separator, Window,
    WindowType,
};
use nalgebra::Vector3;
use std::cell::RefCell;
use std::rc::Rc;

use crate::raytracer::CameraBuilder;
use crate::gui::sections::{
    create_cube_section, create_cylinder_section, create_flat_plane_section, create_sphere_section,
};
use crate::gui::helper::{is_valid_float, update_scene_from_gui};

pub struct AppState {
    pub spheres: Vec<SphereConfig>,
    pub cylinders: Vec<CylinderConfig>,
    pub cubes: Vec<CubeConfig>,
    pub flat_planes: Vec<FlatPlaneConfig>,
}

pub struct SphereConfig {
    pub id: Rc<RefCell<u32>>,
    pub pos_x_entry: Rc<RefCell<Entry>>,
    pub pos_y_entry: Rc<RefCell<Entry>>,
    pub pos_z_entry: Rc<RefCell<Entry>>,
    pub radius_entry: Rc<RefCell<Entry>>,
    pub material_selector: Rc<RefCell<ComboBoxText>>,
    pub color_button: Rc<RefCell<gtk::ColorButton>>,
}
#[derive(Clone)]
pub struct CylinderConfig {
    pub id: Rc<RefCell<u32>>,
    pub pos_x_entry: Rc<RefCell<Entry>>,
    pub pos_y_entry: Rc<RefCell<Entry>>,
    pub pos_z_entry: Rc<RefCell<Entry>>,
    pub radius_entry: Rc<RefCell<Entry>>,
    pub material_selector: Rc<RefCell<ComboBoxText>>,
    pub height_entry: Rc<RefCell<Entry>>,
    pub color_button: Rc<RefCell<gtk::ColorButton>>,
}

pub struct CubeConfig {
    pub id: Rc<RefCell<u32>>,
    pub pos_x_entry: Rc<RefCell<Entry>>,
    pub pos_y_entry: Rc<RefCell<Entry>>,
    pub pos_z_entry: Rc<RefCell<Entry>>,
    pub radius_entry: Rc<RefCell<Entry>>,
    pub  material_selector: Rc<RefCell<ComboBoxText>>,
    pub color_button: Rc<RefCell<gtk::ColorButton>>,
}

pub struct FlatPlaneConfig {
    pub id: Rc<RefCell<u32>>,
    pub pos_x_entry: Rc<RefCell<Entry>>,
    pub pos_y_entry: Rc<RefCell<Entry>>,
    pub pos_z_entry: Rc<RefCell<Entry>>,
    pub radius_entry: Rc<RefCell<Entry>>,
    pub material_selector: Rc<RefCell<ComboBoxText>>,
    pub color_button: Rc<RefCell<gtk::ColorButton>>,
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
