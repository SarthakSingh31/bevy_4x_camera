use bevy::{input::{mouse::{MouseMotion, MouseWheel}}, prelude::*, render::camera::Camera};

pub struct KeyboardConf {
    forward: Box<[KeyCode]>,
    backward: Box<[KeyCode]>,
    left: Box<[KeyCode]>,
    right: Box<[KeyCode]>,
    /// sensitivity is calcualted by mx + c where (m: f32, c: f32)
    /// and x is the camera distance 
    move_sensitivity: (f32, f32),
    clockwise: Box<[KeyCode]>,
    counter_clockwise: Box<[KeyCode]>,
    rotate_sensitivity: f32,
}

impl Default for KeyboardConf {
    fn default() -> Self {
        KeyboardConf {
            forward: Box::new([KeyCode::W, KeyCode::Up]),
            backward: Box::new([KeyCode::S, KeyCode::Down]),
            left: Box::new([KeyCode::A, KeyCode::Left]),
            right: Box::new([KeyCode::D, KeyCode::Right]),
            move_sensitivity: (2.0, 0.1),
            clockwise: Box::new([KeyCode::Q]),
            counter_clockwise: Box::new([KeyCode::E]),
            rotate_sensitivity: std::f32::consts::PI / 100.,
        }
    }
}

pub struct MouseConf {
    rotate: MouseButton,
    rotate_sensitivity: f32,
    drag: MouseButton,
    /// sensitivity is calcualted by mx + c where (m: f32, c: f32)
    /// and x is the camera distance 
    drag_sensitivity: (f32, f32),
    zoom_sensitivity: f32,
}

impl Default for MouseConf {
    fn default() -> Self {
        MouseConf {
            rotate: MouseButton::Right,
            rotate_sensitivity: std::f32::consts::PI / 1000.,
            drag: MouseButton::Left,
            drag_sensitivity: (1., std::f32::consts::PI / 1000.),
            zoom_sensitivity: 5.,
        }
    }
}

/// TODO: Add the ability set more input type here like gamepad
#[derive(Default)]
pub struct CameraRig {
    pub keyboard: KeyboardConf,
    pub mouse: MouseConf,
    // Transforms for (Rig, Camera)
    pub move_to: (Option<Transform>, Option<Transform>),
}

#[derive(Bundle, Default)]
pub struct CameraRigBundle {
    pub camera_rig: CameraRig,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Default)]
struct MouseEventReader {
    motion: EventReader<MouseMotion>,
    wheel: EventReader<MouseWheel>,
}

fn camera_rig_movement(
    mut readers: Local<MouseEventReader>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut rig_query: Query<(&mut Transform, &mut CameraRig, &Children)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut follow_query: Query<&mut CameraRigFollow>
) {
    for (mut rig_transform, mut rig, children) in rig_query.iter_mut() {
        let mut move_to_rig = if let Some(trans) = rig.move_to.0 {
            trans
        } else {
            *rig_transform
        };

        let mut translated = false;
        let move_sensitivity = rig_transform.translation.y *
            rig.keyboard.move_sensitivity.0 +
            rig.keyboard.move_sensitivity.1;
        // Rig Keyboard Movement
        if rig.keyboard.forward.iter().any(|key| keyboard_input.pressed(*key)) {
            move_to_rig.translation += rig_transform.rotation * Vec3::unit_x() * move_sensitivity;
            translated = true;
        }
        if rig.keyboard.backward.iter().any(|key| keyboard_input.pressed(*key)) {
            move_to_rig.translation -= rig_transform.rotation * Vec3::unit_x() * move_sensitivity;
            translated = true;
        }
        if rig.keyboard.right.iter().any(|key| keyboard_input.pressed(*key)) {
            move_to_rig.translation += rig_transform.rotation * Vec3::unit_z() * move_sensitivity;
            translated = true;
        }
        if rig.keyboard.left.iter().any(|key| keyboard_input.pressed(*key)) {
            move_to_rig.translation -= rig_transform.rotation * Vec3::unit_z() * move_sensitivity;
            translated = true;
        }

        // Rig Keyboard Rotation
        if rig.keyboard.counter_clockwise.iter().any(|key| keyboard_input.pressed(*key)) {
            move_to_rig.rotate(Quat::from_rotation_y(rig.keyboard.rotate_sensitivity));
        }
        if rig.keyboard.clockwise.iter().any(|key| keyboard_input.pressed(*key)) {
            move_to_rig.rotate(Quat::from_rotation_y(- rig.keyboard.rotate_sensitivity));
        }

        // Rig Mouse Motion
        let mut mouse_delta_y = 0.;
        for event in readers.motion.iter(&mouse_motion_events) {
            if mouse_input.pressed(rig.mouse.rotate) {
                move_to_rig.rotate(Quat::from_rotation_y(- rig.mouse.rotate_sensitivity * event.delta.x));
                mouse_delta_y += event.delta.y;
            }
            if mouse_input.pressed(rig.mouse.drag) {
                let drag_sensitivity = rig_transform.translation.y *
                    rig.mouse.drag_sensitivity.0 +
                    rig.mouse.drag_sensitivity.1;
                move_to_rig.translation += rig_transform.rotation * Vec3::new(event.delta.y, 0., - event.delta.x) * drag_sensitivity;
                translated = true;
            }
        }

        if translated {
            for mut followable in follow_query.iter_mut() {
                followable.0 = false;
            }
        }

        rig.move_to.0 = Some(move_to_rig);

        // Smoothly move the rig
        if move_to_rig.translation != rig_transform.translation {
            if move_to_rig.translation.distance(rig_transform.translation).abs() > 0.005 {
                rig_transform.translation = rig_transform.translation.lerp(move_to_rig.translation, time.delta().as_micros() as f32 / 100000.);
            } else {
                rig_transform.translation = move_to_rig.translation;
            }
        }
        if move_to_rig.rotation != rig_transform.rotation {
            if !move_to_rig.rotation.abs_diff_eq(rig_transform.rotation, 0.00001) {
                rig_transform.rotation = rig_transform.rotation.lerp(move_to_rig.rotation, time.delta().as_micros() as f32 / 100000.);
            } else {
                rig_transform.rotation = move_to_rig.rotation;
            }
        }

        let mut found_camera_child = false;
        for child in children.iter() {
            if let Ok(mut transform) = camera_query.get_mut(*child) {
                let mut move_to_camera = if let Some(trans) = rig.move_to.1 {
                    trans
                } else {
                    *transform
                };

                // Camera Mouse Zoom
                for event in readers.wheel.iter(&mouse_wheel_events) {
                    move_to_camera.translation -= move_to_camera.forward() * event.y * rig.mouse.zoom_sensitivity;
                }

                // Camera Mouse Rotate
                if mouse_input.pressed(rig.mouse.rotate) {
                    move_to_camera.rotate(Quat::from_rotation_x(-rig.mouse.rotate_sensitivity * mouse_delta_y));
                    move_to_camera.translation = Quat::from_rotation_z(-rig.mouse.rotate_sensitivity * mouse_delta_y) * move_to_camera.translation;
                }

                rig.move_to.1 = Some(move_to_camera);

                // Smoothly move the camera
                if move_to_camera.translation != transform.translation {
                    if move_to_camera.translation.distance(transform.translation).abs() > 0.005 {
                        transform.translation = transform.translation.lerp(move_to_camera.translation, time.delta().as_micros() as f32 / 100000.);
                    } else {
                        transform.translation = move_to_camera.translation;
                    }
                }
                if move_to_camera.rotation != transform.rotation {
                    if !move_to_camera.rotation.abs_diff_eq(transform.rotation, 0.00001) {
                        transform.rotation = transform.rotation.lerp(move_to_camera.rotation, time.delta().as_micros() as f32 / 100000.);
                    } else {
                        transform.rotation = move_to_camera.rotation;
                    }
                }

                found_camera_child = true;
                break
            }
        }

        if !found_camera_child {
            todo!("bevy diagnostics error here / should I panic here?");
        }
    }
}

pub struct CameraRigFollow(pub bool);

fn camera_rig_follow(
    time: Res<Time>,
    mut rig_query: Query<(&mut Transform, &mut CameraRig)>,
    mut follow_query: Query<(&mut Transform, &CameraRigFollow), Changed<Transform>>
) {
    for (follow_transform, follow) in follow_query.iter_mut() {
        if follow.0 {
            for (mut transform, mut rig) in rig_query.iter_mut() {
                if follow_transform.translation != transform.translation {
                    if follow_transform.translation.distance(transform.translation).abs() > 0.005 {
                        transform.translation = transform.translation.lerp(follow_transform.translation, time.delta().as_micros() as f32 / 100000.);
                    } else {
                        transform.translation = follow_transform.translation;
                    }
                }

                // Also update the rig translation
                if let Some(mut rig_transform) = rig.move_to.0.as_mut() {
                    rig_transform.translation = transform.translation;
                }
            }
        }
    }
}

pub struct FourXCameraPlugin;

impl Plugin for FourXCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(camera_rig_movement.system())
            .add_system(camera_rig_follow.system());
    }
}