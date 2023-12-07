use crate::gui::*;
use rand::Rng;

pub fn create_sphere_section(
    app_state: Rc<RefCell<AppState>>,
    flow_box: gtk::FlowBox,
) -> gtk::Widget {
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
    radius_entry.set_text("1.0"); // Set default text
    grid.attach(&radius_entry, 0, 8, 1, 1); // Column 0, Row 8

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Material"));
    grid.attach(&material_label, 0, 9, 1, 1); // Column 0, Row 9

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Diffusive");
    material_selector.append_text("Glossy");
    material_selector.append_text("Reflective");
    material_selector.append_text("Light");
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
    radius_entry.set_text("1.0"); // Set default text
    grid.attach(&radius_entry, 0, 8, 1, 1); // Column 0, Row 8

    let height_label = gtk::Label::new(Some("Height"));
    grid.attach(&height_label, 0, 9, 1, 1); // Column 0, Row 9
    let height_entry = Entry::new();
    height_entry.set_text("2.0"); // Set default text
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
    material_selector.append_text("Diffusive");
    material_selector.append_text("Glossy");
    material_selector.append_text("Reflective");
    material_selector.append_text("Light");
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
    let radius_label = gtk::Label::new(Some("Size"));
    grid.attach(&radius_label, 0, 7, 1, 1); // Column 0, Row 7

    let radius_entry = Entry::new();
    radius_entry.set_text("1.0"); // Set default text
    grid.attach(&radius_entry, 0, 8, 1, 1); // Column 0, Row 8

    // Material Selector Label and ComboBox
    let material_label = gtk::Label::new(Some("Material"));
    grid.attach(&material_label, 0, 9, 1, 1); // Column 0, Row 9

    let material_selector = ComboBoxText::new();
    material_selector.append_text("Diffusive");
    material_selector.append_text("Glossy");
    material_selector.append_text("Reflective");
    material_selector.append_text("Light");
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

pub fn create_flat_plane_section(
    app_state: Rc<RefCell<AppState>>,
    flow_box: gtk::FlowBox,
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
    material_selector.append_text("Diffusive");
    material_selector.append_text("Glossy");
    material_selector.append_text("Reflective");
    material_selector.append_text("Light");
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

    grid.upcast::<gtk::Widget>() // Return the grid as a generic widget
}
