# Ray Tracing

#### Authored by: [Viktor Boman](https://github.com/bomanviktor), [Johannes Eckerman](https://github.com/jo-eman), [Salam Foon](https://github.com/salam-github), [Ville Patjas](https://github.com/Villepat), and [André Teetor](https://github.com/FinnTune)
###### Completed during [grit:lab](https://gritlab.ax/) full-stack development course.

#### Project Description: [rt instructions](https://github.com/01-edu/public/blob/master/subjects/rt/README.md)

## Table of Contents
- [GUI](#gui)
- [Light](#light)
- [Brightness](#brightness)
- [Creating Objects In Code](#creating-objects-in-code)
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
### Sphere

To create a sphere object, use the `Sphere` struct from `sphere.rs`. Here's an example:

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

