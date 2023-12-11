use crate::gui::{CubeConfig, CylinderConfig, FlatPlaneConfig, SphereConfig};
use gtk::EntryExt;

pub fn validate_spheres(spheres: &[SphereConfig]) -> bool {
    for sphere in spheres {
        let pos_x = sphere.pos_x_entry.borrow().get_text().to_string();
        let pos_y = sphere.pos_y_entry.borrow().get_text().to_string();
        let pos_z = sphere.pos_z_entry.borrow().get_text().to_string();
        let radius = sphere.radius_entry.borrow().get_text().to_string();

        if !is_valid_number(&pos_x)
            || !is_valid_number(&pos_y)
            || !is_valid_number(&pos_z)
            || !is_valid_number(&radius)
        {
            return false;
        }
    }
    true
}

pub fn validate_cylinders(cylinders: &[CylinderConfig]) -> bool {
    for cylinder in cylinders.iter() {
        let pos_x = cylinder.pos_x_entry.borrow().get_text().to_string();
        let pos_y = cylinder.pos_y_entry.borrow().get_text().to_string();
        let pos_z = cylinder.pos_z_entry.borrow().get_text().to_string();
        let radius = cylinder.radius_entry.borrow().get_text().to_string();
        let height = cylinder.height_entry.borrow().get_text().to_string();

        if !is_valid_number(&pos_x)
            || !is_valid_number(&pos_y)
            || !is_valid_number(&pos_z)
            || !is_valid_number(&radius)
            || !is_valid_number(&height)
        {
            return false;
        }
    }
    true
}

pub fn validate_cubes(cubes: &[CubeConfig]) -> bool {
    for cube in cubes {
        let pos_x = cube.pos_x_entry.borrow().get_text().to_string();
        let pos_y = cube.pos_y_entry.borrow().get_text().to_string();
        let pos_z = cube.pos_z_entry.borrow().get_text().to_string();
        let radius = cube.radius_entry.borrow().get_text().to_string();

        if !is_valid_number(&pos_x)
            || !is_valid_number(&pos_y)
            || !is_valid_number(&pos_z)
            || !is_valid_number(&radius)
        {
            return false;
        }
    }
    true
}

pub fn validate_flat_planes(flat_planes: &[FlatPlaneConfig]) -> bool {
    for flat_plane in flat_planes {
        let pos_x = flat_plane.pos_x_entry.borrow().get_text().to_string();
        let pos_y = flat_plane.pos_y_entry.borrow().get_text().to_string();
        let pos_z = flat_plane.pos_z_entry.borrow().get_text().to_string();
        let radius = flat_plane.radius_entry.borrow().get_text().to_string();

        if !is_valid_number(&pos_x)
            || !is_valid_number(&pos_y)
            || !is_valid_number(&pos_z)
            || !is_valid_number(&radius)
        {
            return false;
        }
    }
    true
}

// Function to validate position entries
fn is_valid_number(input: &str) -> bool {
    input.is_empty() || input.parse::<f64>().is_ok()
}
