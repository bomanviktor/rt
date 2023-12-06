# Ray Tracing

#### Authored by: [Viktor Boman](https://github.com/bomanviktor), [Johannes Eckerman](https://github.com/jo-eman), [Salam Foon](https://github.com/salam-github), [Ville Patjas](https://github.com/Villepat), and [André Teetor](https://github.com/FinnTune)
###### Completed during [grit:lab](https://gritlab.ax/) full-stack development course.

#### Project Description: [rt instructions](https://github.com/01-edu/public/blob/master/subjects/rt/README.md)

## Table of Contents
- [GUI](#gui)
- [Light](#light)
- [Brightness](#brightness)
- [Creating Objects In Code](#creating-objects-in-code)
    - [Camera Position](#camera-position)
    - [Sphere](#sphere)
    - [Cube](#cube)
    - [Plane](#plane)
    - [Cylinder](#cylinder)


## GUI
The GUI is built using [gtk-rs](https://gtk-rs.org/). It is a simple GUI that allows the user to create objects and change their properties. The GUI also allows the user to change the camera position and rotation, as well as the background color. The GUI also allows the user to change the resolution of the image and the number of samples per pixel.


## Light
The light is hardcoded into the `Scene` and can be moved around thusly in `gui/helper.rs`

```rust
pub fn update_scene_from_gui(app_state: Rc<RefCell<AppState>>) -> Scene {
    let light = Sphere::new(Vector3::new(-5.0, -6.0, -10.0), 2.0, Light(Color::white()));

    objects.push(Arc::new(light));
}
  ```

## Brightness
The brightness of the image can be changed by changing...


## Creating Objects In Code

To create objects in code (without launching the GUI), you can uncomment the following code snippet and dependencies in `main.rs`:
```rust
use rt::gui::launch_gui; // Comment out to run ray tracer without gui
// Uncomment below to run ray tracer without gui
// use std::sync::Arc;
// use std::time::Instant;
// use rt::raytracer::{CameraBuilder, Scene};
// use rt::type_aliases::{Direction, Point};
//
// const OUTPUT_PATH: &str = "output.ppm";

fn main() {
  launch_gui(); // Comment out to run ray tracer without gui
  // Uncomment below to run ray tracer without gui
  // let mut camera = CameraBuilder::new()
  //     .sample_size(20)
  //     .position_by_coordinates(Point::new(-3.0, -4.0, 5.0))
  //     .look_at(Point::new(0.0, 0.0, 0.0))
  //     .up_direction_by_coordinates(Direction::new(0.0, 1.0, 0.0))
  //     .focal_length(0.5)
  //     .sensor_width(1.0)
  //     .build();
  //
  // let scene_data = "scene info from gui here";
  // let scene = Arc::new(Scene::init(scene_data));
  //
  // let start = Instant::now();
  //
  // // Perform ray tracing
  // camera.send_rays(scene.clone());
  //
  // camera.write_to_ppm(OUTPUT_PATH);
  //
  // let duration = start.elapsed();
  // println!("Time taken for rendering: {:?}", duration);
}

```

### Camera Position

To change the camera position, change the following `position_by_coordinates` in `main.rs`:
```rusts
let mut camera = CameraBuilder::new()
                    .sample_size(20)
                    .position_by_coordinates(Point::new(-3.0, -4.0, 5.0))
                    .look_at(Point::new(0.0, 0.0, 0.0))
                    .up_direction_by_coordinates(Direction::new(0.0, 1.0, 0.0))
                    .focal_length(0.5)
                    .sensor_width(1.0)
                    .build();
```


### Sphere

To create a objects, use the below, e.g. `Sphere` struct from `sphere.rs` in `scene.rs`. Here's an example:

```rust
let sphere = Sphere::new(position, radius, material);
```

### Cube
```rust
let cube = Cube::new(min, max, material);
```

### Plane
```rust
let plane = FlatPlane::new(normal, distance, material);
```

### Cylinder
```rust
let cylinder = Cylinder::new(base, axis, radius, material);
```

