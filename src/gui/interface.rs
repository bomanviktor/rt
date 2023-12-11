use crate::gui::components::*;
use crate::gui::GtkBox as Box;
use crate::gui::*;
use crate::raytracer::CameraBuilder;

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

    let vertical_box = GtkBox::new(Orientation::Vertical, 0);
    let top_horizontal_box = GtkBox::new(Orientation::Horizontal, 10);
    vertical_box.set_border_width(10);
    vertical_box.set_spacing(10);

    // About window and button
    let about_dialog = about_dialog(&scaled_icon);
    about_dialog.set_transient_for(Some(&window));
    let about_button = about_btn(&about_dialog, &provider);

    top_horizontal_box.pack_start(&about_button, false, false, 0);

    vertical_box.pack_start(&top_horizontal_box, false, false, 0);
    scrolled_window.add(&vertical_box);
    window.add(&scrolled_window);

    // Sample Size
    let adjustment = gtk::Adjustment::new(5000.0, 1.0, 10000.0, 1.0, 10.0, 0.0);
    let sample_size_scale = horizontal_scale("Sample size", adjustment, &vertical_box);
    sample_size_scale.set_digits(0);

    let dual_scales = gtk::Box::new(Orientation::Horizontal, 0);

    // Brightness
    let brightness_box = gtk::Box::new(Orientation::Vertical, 0);
    let adjustment = gtk::Adjustment::new(0.5, 0.0, 1.0, 0.01, 0.01, 0.0);
    let brightness_scale = horizontal_scale("Brightness", adjustment, &brightness_box);
    brightness_scale.set_value(0.5);
    brightness_scale.set_digits(2);

    // Focal length
    let focal_length_box = gtk::Box::new(Orientation::Vertical, 0);
    let adjustment = gtk::Adjustment::new(1.0, 0.01, 2.0, 0.01, 0.01, 0.0);
    let focal_length_scale = horizontal_scale("Focal length", adjustment, &focal_length_box);
    focal_length_scale.set_value(1.0);
    focal_length_scale.set_digits(2);

    dual_scales.pack_start(&brightness_box, true, true, 0);
    dual_scales.pack_start(&focal_length_box, true, true, 0);
    vertical_box.pack_start(&dual_scales, false, true, 0);

    let app_state_clone = app_state.clone();

    brightness_scale.connect_value_changed(move |scale| {
        let brightness_value = scale.get_value();
        app_state_clone.borrow_mut().brightness = brightness_value;
    });

    separator(&vertical_box, 10);

    let camera_box = gtk::Box::new(Orientation::Horizontal, 5);
    camera_box.set_border_width(10);
    camera_box.set_spacing(10);

    // Camera position
    let (cam_x_entry, cam_y_entry, cam_z_entry) =
        add_coordinate_widgets_box(&camera_box, "Camera position:", ["-6.0", "4.0", "15.0"]);

    // Looking at
    let (look_at_x_entry, look_at_y_entry, look_at_z_entry) =
        add_coordinate_widgets_box(&camera_box, "Looking at:", ["0.0", "0.0", "0.0"]);

    vertical_box.pack_start(&camera_box, false, false, 0);

    separator(&vertical_box, 10);

    // Resolution Selection
    let (width_entry, height_entry) = add_resolution_box(&vertical_box);

    separator(&vertical_box, 10);

    // Create a horizontal box for the side-by-side buttons
    let button_box = gtk::Box::new(Orientation::Horizontal, 5);
    let (add_sphere_btn, add_cylinder_btn, add_cube_btn, add_plane_btn) = create_buttons_row(
        &button_box,
        &provider,
        ["Add Sphere", "Add Cylinder", "Add Cube", "Add Flat Plane"],
    );

    // Add the button box to the vertical box
    vertical_box.pack_start(&button_box, false, false, 0);

    separator(&vertical_box, 10);

    // Create a flow box for all the objects
    let object_box = create_object_box(&vertical_box);

    add_sphere_btn.connect_clicked(clone!(@strong object_box, @strong app_state => move |_| {
        create_sphere_section(app_state.clone(), object_box.clone());
    }));

    add_cylinder_btn.connect_clicked(clone!(@strong object_box, @strong app_state => move |_| {
        create_cylinder_section(app_state.clone(), object_box.clone());
    }));

    add_cube_btn.connect_clicked(clone!(@strong object_box, @strong app_state => move |_| {
        create_cube_section(app_state.clone(), object_box.clone());
    }));

    add_plane_btn.connect_clicked(clone!(@strong object_box, @strong app_state => move |_| {
        create_flat_plane_section(app_state.clone(), object_box.clone());
    }));

    // Create and add the render button to the vertical box
    let render_button = create_button_with_label("Render", &provider);
    vertical_box.pack_start(&render_button, false, false, 0);

    // Define CSS styles for the message label
    let red_style =
        "<span foreground='red'>Invalid input detected. Please enter numbers in 0.0 format.</span>";
    let green_style =
        "<span foreground='green'>All inputs are valid. Proceeding with rendering.</span>";

    let render_done_style = "<span foreground='green'>Rendering done.</span>";

    // Create a label for displaying messages
    let message_label = gtk::Label::new(None);
    message_label.set_text("Ready"); // Default text
    vertical_box.pack_start(&message_label, false, false, 10); // Adjust packing as needed

    let show_image_button = Button::with_label("Show Image");
    vertical_box.pack_start(&show_image_button, false, false, 0);

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


        let sample_size = sample_size_scale.get_value() as u16;
        let focal_length = focal_length_scale.get_value();
        if let (Ok(x), Ok(y), Ok(z), Ok(look_x), Ok(look_y), Ok(look_z), Ok(w), Ok(h)) = (
            cam_x_entry.get_text().parse::<f64>(),
            cam_y_entry.get_text().parse::<f64>(),
            cam_z_entry.get_text().parse::<f64>(),
            look_at_x_entry.get_text().parse::<f64>(),
            look_at_y_entry.get_text().parse::<f64>(),
            look_at_z_entry.get_text().parse::<f64>(),
            width_entry.get_text().parse::<u32>(),
            height_entry.get_text().parse::<u32>(),
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
        let message_label_clone = message_label.clone();

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
                message_label_clone.set_markup(render_done_style);

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

fn create_object_box(vertical_box: &Box) -> FlowBox {
    let flow_box = FlowBox::new();
    flow_box.set_valign(gtk::Align::Start);
    flow_box.set_max_children_per_line(10);
    flow_box.set_selection_mode(gtk::SelectionMode::None);
    vertical_box.pack_start(&flow_box, false, false, 0);
    flow_box
}

fn separator(vertical_box: &Box, padding: u32) {
    let separator = Separator::new(Orientation::Horizontal);
    vertical_box.pack_start(&separator, false, false, padding);
}
