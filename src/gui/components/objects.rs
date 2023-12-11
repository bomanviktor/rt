use crate::gui::components::entries::*;
use crate::gui::*;
use rand::Rng;

const MATERIALS: [&str; 3] = ["Diffusive", "Reflective", "Light"];
fn append_materials(material_selector: &ComboBoxText) {
    for material in MATERIALS {
        material_selector.append_text(material);
    }
    material_selector.set_active(Some(0));
}

fn delete_component(flow_box: &FlowBox, id: String) {
    for child in flow_box.get_children().iter() {
        // Attempt to downcast the child to GtkFlowBoxChild
        if let Some(flow_box_child) = child.downcast_ref::<gtk::FlowBoxChild>() {
            if let Some(widget) = flow_box_child.get_child() {
                let widget_name = widget.get_widget_name().to_string(); // Get the name of the widget inside the GtkFlowBoxChild
                if widget_name == id {
                    flow_box.remove(child);
                    break;
                }
            }
        }
    }
}

pub fn create_sphere_section(app_state: Rc<RefCell<AppState>>, flow_box: FlowBox) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let sphere_count = app_state.borrow().spheres.len() + 1;
    let unique_id = format!("sphere_{}", sphere_count); // Generate unique ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid
    print!("Setting grid ID: '{}'", grid.get_widget_name()); // Debug print for grid ID
    println!("Grid set with widget name: {}", unique_id); // Debug print for grid ID

    let placeholders = vec!["0.0", "0.0", "0.0", "1.0"];
    let sphere_entries = add_coordinate_widgets_grid(
        &grid,
        &provider,
        &format!("Sphere {sphere_count}:"),
        &placeholders,
    );
    let pos_x_entry = &sphere_entries[0];
    let pos_y_entry = &sphere_entries[1];
    let pos_z_entry = &sphere_entries[2];
    let radius_entry = &sphere_entries[3];

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Texture"));
    grid.attach(&material_label, 0, 9, 1, 1); // Column 0, Row 9

    let material_selector = ComboBoxText::new();
    append_materials(&material_selector);

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
    grid.attach(&material_selector, 0, 10, 1, 1); // Column 0, Row 10

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 11, 1, 1); // Column 0, Row 11

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 12, 1, 1); // Column 0, Row 12

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
        radius_entry_clone.set_text(&format!("{:.2}", rng.gen_range(0.1..1.0)));

        // Randomly select a material
        let random_material_index = rng.gen_range(0..MATERIALS.len());
        material_selector_clone.set_active(Some(random_material_index as u32));
    });

    // Create a delete button for the sphere section
    let delete_id = sphere_config.id.clone();
    let delete_button = Button::with_label("Delete");
    println!("Adding delete button with ID: {}", *delete_id.borrow());
    grid.attach(&delete_button, 0, 14, 1, 1); // Column 0, Row 13

    // Connect the delete button click handler
    delete_button.connect_clicked(clone!(@strong app_state, @strong flow_box => move |_| {
    let id = format!("sphere_{}", *delete_id.borrow());

    let mut app_state = app_state.borrow_mut();
    if let Some(index) = app_state.spheres.iter().position(|s| format!("sphere_{}", *s.id.borrow()) == id) {
        app_state.spheres.remove(index);
    } else {
        eprintln!("Error: No sphere with ID {} found in app_state", id);
        return;
    }
    delete_component(&flow_box, id);
    flow_box.show_all();
}));

    app_state.borrow_mut().spheres.push(sphere_config);

    flow_box.add(&grid); // Directly add the grid to the flow_box
    flow_box.show_all();
    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

pub fn create_cylinder_section(
    app_state: Rc<RefCell<AppState>>,
    flow_box: gtk::FlowBox,
) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let cylinder_count = app_state.borrow().cylinders.len() + 1;
    let unique_id = format!("cylinder_{}", cylinder_count); // Generate unique ID
    println!("Creating cylinder section with ID: {}", unique_id); // Debug print for cylinder ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid
    let placeholders = vec!["0.0", "0.0", "0.0", "1.0", "2.0"];
    let cylinder_entries = add_coordinate_widgets_grid(
        &grid,
        &provider,
        &format!("Cylinder {cylinder_count}:"),
        &placeholders,
    );
    let pos_x_entry = &cylinder_entries[0];
    let pos_y_entry = &cylinder_entries[1];
    let pos_z_entry = &cylinder_entries[2];
    let radius_entry = &cylinder_entries[3];
    let height_entry = &cylinder_entries[4];

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Texture"));
    grid.attach(&material_label, 0, 11, 1, 1);

    let material_selector = ComboBoxText::new();
    append_materials(&material_selector);
    grid.attach(&material_selector, 0, 12, 1, 1);

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 13, 1, 1);

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 14, 1, 1);

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
        let random_material_index = rng.gen_range(0..MATERIALS.len());
        material_selector_clone.set_active(Some(random_material_index as u32));
    });

    // Create a delete button for the cylinder section
    let delete_id = cylinder_config.id.clone();
    let delete_button = gtk::Button::with_label("Delete");
    grid.attach(&delete_button, 0, 16, 1, 1); // Column 0, Row 15

    // Connect the delete button click handler
    delete_button.connect_clicked(clone!(@strong app_state, @strong flow_box => move |_| {
        let id = format!("cylinder_{}", *delete_id.borrow());

        let mut app_state = app_state.borrow_mut();
        if let Some(index) = app_state.cylinders.iter().position(|c| format!("cylinder_{}", *c.id.borrow()) == id) {
            app_state.cylinders.remove(index);
        } else {
            eprintln!("Error: No cylinder with ID {} found in app_state", id);
            return;
        }
        delete_component(&flow_box, id);
        flow_box.show_all();
    }));

    // Add the configuration to the AppState
    app_state.borrow_mut().cylinders.push(cylinder_config);

    flow_box.add(&grid);
    flow_box.show_all();
    print!("Added GUI element with ID: '{}'", unique_id); // Debug print for cylinder ID
    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

pub fn create_cube_section(
    app_state: Rc<RefCell<AppState>>,
    flow_box: gtk::FlowBox,
) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let cube_count = app_state.borrow().cubes.len() + 1;
    let unique_id = format!("cube_{}", cube_count); // Generate unique ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid

    let placeholders = vec!["0.0", "0.0", "0.0", "1.0"];
    let cube_entries = add_coordinate_widgets_grid(
        &grid,
        &provider,
        &format!("Cube {cube_count}:"),
        &placeholders,
    );
    let pos_x_entry = &cube_entries[0];
    let pos_y_entry = &cube_entries[1];
    let pos_z_entry = &cube_entries[2];
    let radius_entry = &cube_entries[3];

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Texture"));
    grid.attach(&material_label, 0, 9, 1, 1); // Column 0, Row 9

    let material_selector = ComboBoxText::new();
    append_materials(&material_selector);
    grid.attach(&material_selector, 0, 10, 1, 1); // Column 0, Row 10

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 11, 1, 1); // Column 0, Row 11

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 12, 1, 1); // Column 0, Row 12

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
        let random_material_index = rng.gen_range(0..MATERIALS.len());
        material_selector_clone.set_active(Some(random_material_index as u32));
    });
    let delete_id = cube_config.id.clone();
    let delete_button = gtk::Button::with_label("Delete");
    grid.attach(&delete_button, 0, 14, 1, 1); //Column 0, Row 13

    // Connect a handler to the delete button
    delete_button.connect_clicked(clone!(@strong app_state, @strong flow_box => move |_| {
        let id = format!("cube_{}", *delete_id.borrow());
        let mut app_state = app_state.borrow_mut();
        if let Some(index) = app_state.cubes.iter().position(|c| format!("cube_{}", *c.id.borrow()) == id) {
            app_state.cubes.remove(index);
        } else {
            eprintln!("Error: No cube with ID {} found in app_state", id);
            return;
        }
        delete_component(&flow_box, id);
        flow_box.show_all();
    }));

    app_state.borrow_mut().cubes.push(cube_config);

    flow_box.add(&grid); // Directly add the grid to the flow_box
    flow_box.show_all();
    print!("Added GUI element with ID: '{}'", unique_id); // Debug print for cube ID
    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}

pub fn create_flat_plane_section(
    app_state: Rc<RefCell<AppState>>,
    flow_box: FlowBox,
) -> gtk::Widget {
    let provider = CssProvider::new();
    provider
        .load_from_path("src/gui/style.css")
        .expect("Failed to load CSS");

    let flat_plane_count = app_state.borrow().flat_planes.len() + 1;
    let unique_id = format!("flat_plane_{}", flat_plane_count); // Generate unique ID

    let grid = gtk::Grid::new();
    grid.set_column_spacing(5); // Adjust the spacing as needed
    grid.set_widget_name(&unique_id); // Set the ID of the grid

    let placeholders = vec!["0.0", "0.0", "0.0", "10.0"];
    let flat_plane_entries = add_coordinate_widgets_grid(
        &grid,
        &provider,
        &format!("Flat Plane {flat_plane_count}:"),
        &placeholders,
    );
    let pos_x_entry = &flat_plane_entries[0];
    let pos_y_entry = &flat_plane_entries[1];
    let pos_z_entry = &flat_plane_entries[2];
    let radius_entry = &flat_plane_entries[3];

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Texture"));
    grid.attach(&material_label, 0, 9, 1, 1);

    let material_selector = ComboBoxText::new();
    append_materials(&material_selector);
    grid.attach(&material_selector, 0, 10, 1, 1);

    let style_context = material_selector.get_style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    // Color Button Label and ColorPicker
    let color_label = gtk::Label::new(Some("Color"));
    grid.attach(&color_label, 0, 11, 1, 1);

    let color_button = gtk::ColorButton::new();
    grid.attach(&color_button, 0, 12, 1, 1);

    grid.set_widget_name(&unique_id);

    let flat_plane_config = FlatPlaneConfig {
        id: Rc::new(RefCell::new(flat_plane_count as u32)),
        pos_x_entry: Rc::new(RefCell::new(pos_x_entry.clone())),
        pos_y_entry: Rc::new(RefCell::new(pos_y_entry.clone())),
        pos_z_entry: Rc::new(RefCell::new(pos_z_entry.clone())),
        radius_entry: Rc::new(RefCell::new(radius_entry.clone())),
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

        let mut app_state = app_state.borrow_mut();
        if let Some(index) = app_state.flat_planes.iter().position(|fp| format!("flat_plane_{}", *fp.id.borrow()) == id) {
            app_state.flat_planes.remove(index);
        } else {
            eprintln!("Error: No flat_plane with ID {} found in app_state", id);
            return;
        }

        delete_component(&flow_box, id);

        flow_box.show_all();
    }));

    app_state.borrow_mut().flat_planes.push(flat_plane_config);

    flow_box.add(&grid); // Directly add the grid to the flow_box
    flow_box.show_all();

    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}
