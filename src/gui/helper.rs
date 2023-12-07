use crate::color::RGB;
use crate::gui::AppState;
use crate::objects::{Cube, Cylinder, FlatPlane, Objects, Sphere};
use crate::raytracer::Scene;
use crate::textures::Texture::*;
use crate::type_aliases::Color;
use gtk::{ColorChooserExt, ComboBoxTextExt, EntryExt};
use nalgebra::Vector3;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

// Function to validate position entries
pub fn is_valid_float(input: &str) -> bool {
    // Check if the input is a valid floating-point number
    let is_float = input.parse::<f64>().is_ok();

    // Check if the input contains a decimal point
    let has_decimal_point = input.contains('.');

    // The input is valid if it's a float and contains a decimal point
    is_float && has_decimal_point
}

pub fn update_scene_from_gui(app_state: Rc<RefCell<AppState>>) -> Scene {
    let app_state_borrowed = app_state.borrow();
    let mut objects: Objects = Vec::new();

    let light = Sphere::new(Vector3::new(-5.0, 10.0, -10.0), 2.0, Light(Color::white()));

    objects.push(Arc::new(light));

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
        let material = sphere_config
            .material_selector
            .borrow()
            .get_active_text()
            .unwrap_or_else(|| "DefaultMaterial".into());

        let color = sphere_config.color_button.borrow().get_rgba();

        let sphere_color = Color::new(color.red * 255., color.green * 255., color.blue * 255.);

        let sphere_texture = match material.as_str() {
            "Diffusive" => Diffusive(sphere_color),
            "Reflective" => Reflective,
            "Glossy" => Glossy(sphere_color),
            // Add other cases as needed
            _ => Diffusive(sphere_color), // Default case
        };

        let sphere = Sphere::new(Vector3::new(pos_x, pos_y, pos_z), radius, sphere_texture);
        objects.push(Arc::new(sphere));
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
        let material = cylinder_config
            .material_selector
            .borrow()
            .get_active_text()
            .unwrap_or_else(|| "DefaultMaterial".into());

        let cylinder_color = Color::new(color.red * 255., color.green * 255., color.blue * 255.);

        let cylinder_texture = match material.as_str() {
            "Diffusive" => Diffusive(cylinder_color),
            "Reflective" => Reflective,
            "Glossy" => Glossy(cylinder_color),
            // Add other cases as needed
            _ => Diffusive(cylinder_color), // Default case
        };

        let cylinder = Cylinder::new(
            Vector3::new(pos_x, pos_y, pos_z),
            radius,
            height,
            cylinder_texture,
        );
        objects.push(Arc::new(cylinder));
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
        let material = cube_config
            .material_selector
            .borrow()
            .get_active_text()
            .unwrap_or_else(|| "DefaultMaterial".into());

        let cube_color = Color::new(color.red * 255., color.green * 255., color.blue * 255.);

        let cube_texture = match material.as_str() {
            "Diffusive" => Diffusive(cube_color),
            "Reflective" => Reflective,
            "Glossy" => Glossy(cube_color),
            // Add other cases as needed
            _ => Diffusive(cube_color), // Default case
        };

        let cube = Cube::new(Vector3::new(pos_x, pos_y, pos_z), radius, cube_texture);
        objects.push(Arc::new(cube));
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
        let material = flat_plane_config
            .material_selector
            .borrow()
            .get_active_text()
            .unwrap_or_else(|| "DefaultMaterial".into());

        let flat_plane_color = Color::new(color.red * 255., color.green * 255., color.blue * 255.);

        let flat_plane_texture = match material.as_str() {
            "Diffusive" => Diffusive(flat_plane_color),
            "Reflective" => Reflective,
            "Glossy" => Glossy(flat_plane_color),
            // Add other cases as needed
            _ => Diffusive(flat_plane_color), // Default case
        };

        let flat_plane = FlatPlane::new(
            Vector3::new(pos_x, pos_y, pos_z),
            radius,
            flat_plane_texture,
        );
        objects.push(Arc::new(flat_plane));
    }

    Scene {
        objects,
        brightness: app_state_borrowed.brightness,
    }
}
