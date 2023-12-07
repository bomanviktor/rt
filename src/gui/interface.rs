use crate::gui::components::*;
use crate::gui::components::{add_coordinate_widgets, add_resolution_box};
use crate::gui::*;
use crate::raytracer::CameraBuilder;

pub struct AppState {
    pub spheres: Vec<SphereConfig>,
    pub cylinders: Vec<CylinderConfig>,
    pub cubes: Vec<CubeConfig>,
    pub flat_planes: Vec<FlatPlaneConfig>,
    pub brightness: f64,
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
    pub material_selector: Rc<RefCell<ComboBoxText>>,
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

pub fn launch_gui() {
    let app_state = Rc::new(RefCell::new(AppState {
        spheres: Vec::new(),
        cylinders: Vec::new(),
        cubes: Vec::new(),
        flat_planes: Vec::new(),
        brightness: 0.5,
    }));

    gtk::init().expect("Failed to initialize GTK.");

    // Create a CSS provider
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let window = Window::new(WindowType::Toplevel);
    window.set_resizable(false); // Allows the window to be resized
    window.set_decorated(true); // Allows the window to be decorated
    window.set_title("Ray Trace Settings");
    window.set_default_size(800, 1000);
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
        Err(err) => std::eprintln!("Failed to load icon: {}", err),
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

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    vbox.pack_start(&separator, false, false, 10);

    // Sample Size
    let sample_size_label = gtk::Label::new(Some("Sample Size"));
    sample_size_label
        .get_style_context()
        .add_class("sample-size-label");
    vbox.pack_start(&sample_size_label, false, false, 0);

    let adjustment = gtk::Adjustment::new(1000.0, 1.0, 10000.0, 1.0, 10.0, 0.0);
    let sample_size_scale = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
    sample_size_scale.set_digits(0); // No decimal places
    sample_size_scale.set_hexpand(true);
    sample_size_scale.set_valign(gtk::Align::Start);
    sample_size_scale.connect_scroll_event(|_, _| {
        Inhibit(true) // This prevents the scale from being adjusted with the mouse scroll
    });
    vbox.pack_start(&sample_size_scale, false, true, 0);

    // Brightness
    let adjustment = gtk::Adjustment::new(
        0.5,  // initial value
        0.0,  // minimum value
        1.0,  // maximum value
        0.01, // step increment
        0.01, // page increment
        0.0,  // page size
    );
    let brightness_label = gtk::Label::new(Some("Brightness"));
    vbox.pack_start(&brightness_label, false, false, 0);

    let brightness_entry = Scale::new(Orientation::Horizontal, Some(&adjustment));
    brightness_entry.set_value(0.5); // Set a default value
    brightness_entry.set_digits(2); // Set a default value
    brightness_entry.connect_scroll_event(|_, _| {
        Inhibit(true) // This prevents the scale from being adjusted with the mouse scroll
    });
    vbox.pack_start(&brightness_entry, false, false, 0);

    let app_state_clone = app_state.clone();

    brightness_entry.connect_value_changed(move |scale| {
        let brightness_value = scale.get_value();
        app_state_clone.borrow_mut().brightness = brightness_value;
        // Optionally, you can also do some real-time updates or logging here
        std::println!("Brightness adjusted to: {}", brightness_value);
    });
    //////////////////////////////////////////////////////////////////////////
    // Camera Options
    // Focal length
    let focal_length_label = gtk::Label::new(Some("Focal length"));
    focal_length_label
        .get_style_context()
        .add_class("sample-size-label");
    vbox.pack_start(&focal_length_label, false, false, 0);

    let adjustment = gtk::Adjustment::new(1.0, 0.1, 5.0, 0.01, 0.01, 0.0);
    let focal_length_scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
    focal_length_scale.set_value(1.0);
    focal_length_scale.set_digits(2);
    focal_length_scale.set_hexpand(true);
    focal_length_scale.set_valign(gtk::Align::Start);
    focal_length_scale.connect_scroll_event(|_, _| {
        Inhibit(true) // This prevents the scale from being adjusted with the mouse scroll
    });
    vbox.pack_start(&focal_length_scale, false, true, 0);

    // Camera position
    let (cam_x_entry, cam_y_entry, cam_z_entry) = add_coordinate_widgets(
        &vbox,
        "Camera Position:",
        ["Default: 0.0", "Default: 0.0", "Default: 0.0"],
    );

    // Looking at
    let (look_at_x_entry, look_at_y_entry, look_at_z_entry) = add_coordinate_widgets(
        &vbox,
        "Looking at:",
        ["Default: 0.0", "Default: 0.0", "Default: 0.0"],
    );

    /*
           cam_x_entry_clone.get_text().parse::<f64>(),
           cam_y_entry_clone.get_text().parse::<f64>(),
           cam_z_entry_clone.get_text().parse::<f64>(),
           looking_at_x_entry.get_text().parse::<f64>(),
           looking_at_y_entry.get_text().parse::<f64>(),
           looking_at_z_entry.get_text().parse::<f64>(),
           width_entry_clone.get_text().parse::<u32>(),
           height_entry_clone.get_text().parse::<u32>(),
    */

    let flow_box = gtk::FlowBox::new();
    let resolution_hbox = gtk::Box::new(Orientation::Horizontal, 5);

    // Resolution Selection
    let (width_entry, height_entry) = add_resolution_box(&vbox, &flow_box, &resolution_hbox);

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

    // Add the horizontal box to the vertical box
    vbox.pack_start(&hbox, false, false, 0);

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

    let sample_size_scale_clone = sample_size_scale.clone();
    let cam_x_entry_clone = cam_x_entry.clone();
    let cam_y_entry_clone = cam_y_entry.clone();
    let cam_z_entry_clone = cam_z_entry.clone();
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
        let app_state_borrowed = app_state.borrow();
        let mut all_inputs_valid = true;

        if !validate_spheres(&app_state_borrowed.spheres) {
            message_label.set_markup("<span foreground='red'>Invalid sphere detected.</span>");
            return;
        }

        if !validate_cylinders(&app_state_borrowed.cylinders) {
            message_label.set_markup("<span foreground='red'>Invalid cylinder detected.</span>");
            return;
        }

        if !validate_cubes(&app_state_borrowed.cubes) {
            message_label.set_markup("<span foreground='red'>Invalid cube detected.</span>");
            return;
        }

        if !validate_flat_planes(&app_state_borrowed.flat_planes) {
            message_label.set_markup("<span foreground='red'>Invalid flat plane detected.</span>");
            return;
        }

        let mut cam_x = 0.0;
        let mut cam_y = 0.0;
        let mut cam_z = 0.0;
        let mut look_at_x = 0.0;
        let mut look_at_y = 0.0;
        let mut look_at_z = 0.0;
        let mut width = 0;
        let mut height = 0;


        let sample_size = sample_size_scale_clone.get_value() as u16;
        let focal_length = focal_length_scale.get_value();
        if let (Ok(x), Ok(y), Ok(z), Ok(look_x), Ok(look_y), Ok(look_z), Ok(w), Ok(h)) = (
            cam_x_entry_clone.get_text().parse::<f64>(),
            cam_y_entry_clone.get_text().parse::<f64>(),
            cam_z_entry_clone.get_text().parse::<f64>(),
            look_at_x_entry.get_text().parse::<f64>(),
            look_at_y_entry.get_text().parse::<f64>(),
            look_at_z_entry.get_text().parse::<f64>(),
            width_entry_clone.get_text().parse::<u32>(),
            height_entry_clone.get_text().parse::<u32>(),
        ) {
            cam_x = x;
            cam_y = y;
            cam_z = z;
            look_at_x = look_x;
            look_at_y = look_y;
            look_at_z = look_z;
            width = w;
            height = h;
        } else {
            all_inputs_valid = false;
        }

        if all_inputs_valid {
            println!("All inputs are valid. Proceeding with rendering.");
            message_label.set_markup(green_style);

            // Schedule rendering to start after a short delay
            glib::timeout_add_local(50, clone!(@strong app_state => move || {
                const OUTPUT_PATH: &str = "output.ppm";
                let updated_scene = Arc::new(update_scene_from_gui(app_state.clone()));

                let mut camera = CameraBuilder::new()
                .sample_size(sample_size)
                .position_by_coordinates(Vector3::new(cam_x, cam_y, cam_z))
                .look_at(Vector3::new(look_at_x, look_at_y, look_at_z))
                .focal_length(focal_length)
                .resolution(width, height)
                .sensor_width(1.0)
                .build();

                camera.send_rays(updated_scene);
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
