# rt

#### Authored by: [Viktor Boman](https://github.com/bomanviktor), [Johannes Eckerman](https://github.com/jo-eman), [Salam Foon](https://github.com/salam-github), [Ville Patjas](https://github.com/Villepat), and [André Teetor](https://github.com/FinnTune)
###### Completed during [grit:lab](https://gritlab.ax/) full-stack development course.
#### Project Description: [here](https://github.com/01-edu/public/blob/master/subjects/rt/README.md)

![closeup](https://github.com/bomanviktor/rt/assets/72476579/115a7a5e-e942-46e9-a75f-2246820571cf)

## Table of Contents
- [About](#about)
- [System requirements](#system-requirements)
- [Dependencies](#dependencies)
- [Installation/Running Instructions](#installationrunning-instructions)
- [Features](#features)
- [Run without GUI](#run-without-gui)
    - [Camera settings](#camera-settings)
    - [Brightness](#brightness)
    - [Objects](#objects)
    - [Textures](#textures)
    - [Colors](#colors)
    - [Finalize the scene](#finalize-the-scene)

## About
This is a Monte Carlo based ray tracer with a GUI written from scratch, entirely in [Rust](https://www.rust-lang.org/).
For the GUI the [GTK3](https://docs.gtk.org/gtk3/) library for rust was used.

## System requirements
Unix based OS such as Linux or Mac OS

## Dependencies
- [Rust](https://www.rust-lang.org/)
- [GTK3](https://docs.gtk.org/gtk3/)

## Installation/Running Instructions
1. Clone the repo
2. Install brew
     - `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
3. Install Rust
    - `brew install rust`
4. Install gtk-rs dependencies
    - `brew install gtk+3`
5. Run in repo root
    - `cargo run --release`

_or_

1. Run `./install.sh`




## Features
- Four shapes: `Cube`, `Sphere`, `Flat plane` and `Cylinder`.
- Four materials: `Diffusive`, `Glossy`, `Reflective` and `Light`.
- Ability to change ambient brightness by changing the `brightness` value.
- Rayon multithreading for faster rendering 🚀

## Run without GUI

To run program without launching the GUI:
```cargo run --release no-gui```

### Camera Settings

To change the sample size, camera position, focal length, looking at and resolution, change the following in `main.rs`:
```rust
let mut camera = CameraBuilder::new()
                    .sample_size(20)
                    .position_by_coordinates(Point::new(-6.0, 4.0, 15.0))
                    .focal_length(1.0)
                    .look_at(Point::new(0.0, 0.0, 0.0))
                    .resolution(1920, 1080)
                    .build();
```

### Brightness
```rust
 let scene = Arc::new(Scene::init(0.01)); // Change the 0.01 to a value between 0.0 and 1.0. 1.0 being max, 0.0 being min.
```

### Objects

To create the objects, go to `scene.rs` to initialize the objects, and add them to the `objects` vector using `Arc::new()`:
To create a objects, use the below, e.g. `Sphere` struct from `sphere.rs` in `scene.rs`. Here's an example:

```rust
let sphere = Sphere::new(position, radius, texture);
let cube = Cube::new(position, side_length, texture);
let plane = FlatPlane::new(position, radius, texture);
let cylinder = Cylinder::new(position, radius, height, texture);
```

### Textures
```rust
Diffusive(color)
Light(color)
Reflective
```

### Colors
There are a wide range of colors to choose from. These are just a small sample of all the available colors.
```rust
RGB::new() // Custom color in 255,255,255 format
RGB::random()
RGB::red()
RGB::green()
RGB::blue()
```

### Finalize the scene
```rust
// Initialize an object
let sphere = Sphere::new(
    Point::new(0.0, 1.0, 0.0),
    1.0, 
    Textures::Diffusive(RGB::red())
);

// more objects here...

// Add the objects to the scene in this vector
let objects: Objects = vec![
    Arc::new(sphere),
    Arc::(object2),
    // More objects here...
];

// Return the scene
Scene {
    objects, brightness
}
```



