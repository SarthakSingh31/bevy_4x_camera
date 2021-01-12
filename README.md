[![Crates.io](https://img.shields.io/crates/v/bevy_4x_camera)](https://crates.io/crates/bevy_4x_camera)

A 4X style camera for bevy. [Demo](https://imgur.com/XIIDcIW)

Default Key Bindings:

- W / A / S / D / Arrow Keys / Mouse Left - Move along the horizontal plane
- Q / E / Mouse Right - Rotate around the center
- Mouse Wheel - Zoom

# Example

```rust
use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_4x_camera::{CameraRigBundle, FourXCameraPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FourXCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // camera
        .spawn(CameraRigBundle::default())
        .with_children(|cb| {
            cb.spawn(Camera3dBundle {
                // I recommend setting the fov to a low value to get a
                // a pseudo-orthographic perspective
                perspective_projection: PerspectiveProjection {
                    fov: 0.1,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-20.0, 20., 0.0))
                    .looking_at(Vec3::zero(), Vec3::unit_y()),
                ..Default::default()
            });
        });
}
```

---

# Version Matching

| Bevy Version | `bevy_4x_camera` Version |
| ------------ | ------------------------ |
| `0.4.0`      | `0.1.0`                  |