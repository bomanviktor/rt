# Ray Tracing

#### Authored by: [Viktor Boman](https://github.com/bomanviktor), [Johannes Eckerman](https://github.com/jo-eman), [Salam Foon](https://github.com/salam-github), [Ville Patjas](https://github.com/Villepat), and [André Teetor](https://github.com/FinnTune)
###### Completed during [grit:lab](https://gritlab.ax/) full-stack development course.

#### Project Description: [rt instructions](https://github.com/01-edu/public/blob/master/subjects/rt/README.md)

## Creating Objects
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

