use bevy::{input::{mouse::{MouseMotion, MouseWheel}}, prelude::*, render::{
        camera::{
            Camera,
            camera_system,
            CameraProjection,
            DepthCalculation,
            OrthographicProjection,
            PerspectiveProjection,
            VisibleEntities,
        },
        render_graph::base,
    }};

#[derive(Debug, Clone)]
pub enum Projection {
    // TODO: Make Orthographic camera actually useful
    Orthographic(OrthographicProjection),
    Perspective(PerspectiveProjection),
}

impl Default for Projection {
    fn default() -> Self {
        Projection::Perspective(PerspectiveProjection {
            fov: 0.1,
            ..Default::default()
        })
    }
}

impl CameraProjection for Projection {
    fn get_projection_matrix(&self) -> Mat4 {
        match self {
            Projection::Orthographic(p) => p.get_projection_matrix(),
            Projection::Perspective(p) => p.get_projection_matrix(),
        }
    }
    fn update(&mut self, width: f32, height: f32) {
        match self {
            Projection::Orthographic(p) => p.update(width, height),
            Projection::Perspective(p) => p.update(width, height),
        }
    }
    fn depth_calculation(&self) -> DepthCalculation {
        match self {
            Projection::Orthographic(p) => p.depth_calculation(),
            Projection::Perspective(p) => p.depth_calculation(),
        }
    }
}

/// TODO: Add the ability set more input type here like gamepad
pub struct StrategyCamera {
    pub mouse_rot: MouseButton,
    pub mouse_rot_sensitivity: f32,
    pub mouse_zoom_sensitiviy: f32,
    pub transform: Option<Transform>,
}

impl Default for StrategyCamera {
    fn default() -> Self {
        StrategyCamera {
            mouse_rot: MouseButton::Right,
            mouse_rot_sensitivity: std::f32::consts::PI / 1000.,
            mouse_zoom_sensitiviy: 5.,
            transform: None,
        }
    }
}

/// A bundle to create a strategy camera
#[derive(Bundle)]
pub struct StrategyCameraBundle {
    pub camera: Camera,
    pub projection: Projection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub strategy_camera: StrategyCamera,
}

impl Default for StrategyCameraBundle {
    fn default() -> Self {
        StrategyCameraBundle {
            camera: Camera {
                name: Some(base::camera::CAMERA_3D.to_string()),
                ..Default::default()
            },
            projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            strategy_camera: StrategyCamera::default(),
        }
    }
}

pub struct StrategyCameraRig {
    pub move_forward: &'static [KeyCode],
    pub move_backward: &'static [KeyCode],
    pub move_left: &'static [KeyCode],
    pub move_right: &'static [KeyCode],
    pub move_sensitivity: f32,
    pub rot_left: &'static [KeyCode],
    pub rot_right: &'static [KeyCode],
    pub rot_sensitivity: f32,
    pub mouse_rot: MouseButton,
    pub mouse_rot_sensitivity: f32,
    pub mouse_drag: MouseButton,
    pub mouse_drag_sensitivity: f32,
    pub transform: Option<Transform>,
}

impl Default for StrategyCameraRig {
    fn default() -> Self {
        StrategyCameraRig {
            move_forward: &[KeyCode::W, KeyCode::Up],
            move_backward: &[KeyCode::S, KeyCode::Down],
            move_left: &[KeyCode::A, KeyCode::Left],
            move_right: &[KeyCode::D, KeyCode::Right],
            move_sensitivity: 0.1,
            rot_left: &[KeyCode::Q],
            rot_right: &[KeyCode::E],
            rot_sensitivity: std::f32::consts::PI / 100.,
            mouse_rot: MouseButton::Right,
            mouse_rot_sensitivity: std::f32::consts::PI / 1000.,
            mouse_drag: MouseButton::Left,
            mouse_drag_sensitivity: 0.01,
            transform: None,
        }
    }
}

#[derive(Bundle, Default)]
pub struct StrategyCameraRigBundle {
    pub rig: StrategyCameraRig,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

pub struct StrategyCameraPlugin;

impl Plugin for StrategyCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            stage::POST_UPDATE,
            camera_system::<Projection>.system()
        )
        .add_system(strategy_camera_rig_movement.system())
        .add_system(strategy_camera.system())
        .add_system(strategy_camera_follow.system())
        .add_system(debug_camera_pos.system());
    }
}

#[derive(Default)]
struct MouseWheelEventReader(EventReader<MouseWheel>);
#[derive(Default)]
struct MouseMotionEventReader(EventReader<MouseMotion>);

fn strategy_camera_rig_movement(
    mut reader: Local<MouseMotionEventReader>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
    events: Res<Events<MouseMotion>>,
    mut query: Query<(&mut Transform, &mut StrategyCameraRig)>
) {
    for (mut transform, mut rig_conf) in query.iter_mut() {
        if rig_conf.transform == None {
            rig_conf.transform = Some(transform.clone());
        }
        let mut new_transform = rig_conf.transform.unwrap();

        if rig_conf.move_forward.iter().any(|key| keyboard_input.pressed(*key)) {
            new_transform.translation += transform.rotation * Vec3::unit_x() * rig_conf.move_sensitivity;
        }
        if rig_conf.move_backward.iter().any(|key| keyboard_input.pressed(*key)) {
            new_transform.translation -= transform.rotation * Vec3::unit_x() * rig_conf.move_sensitivity;
        }
        if rig_conf.move_right.iter().any(|key| keyboard_input.pressed(*key)) {
            new_transform.translation += transform.rotation * Vec3::unit_z() * rig_conf.move_sensitivity;
        }
        if rig_conf.move_left.iter().any(|key| keyboard_input.pressed(*key)) {
            new_transform.translation -= transform.rotation * Vec3::unit_z() * rig_conf.move_sensitivity;
        }
        if rig_conf.rot_right.iter().any(|key| keyboard_input.pressed(*key)) {
            new_transform.rotate(Quat::from_rotation_y(- rig_conf.rot_sensitivity));
        }
        if rig_conf.rot_left.iter().any(|key| keyboard_input.pressed(*key)) {
            new_transform.rotate(Quat::from_rotation_y(rig_conf.rot_sensitivity));
        }

        for event in reader.0.iter(&events) {
            if mouse_button.pressed(rig_conf.mouse_rot) {
                new_transform.rotate(Quat::from_rotation_y(- rig_conf.mouse_rot_sensitivity * event.delta.x));
            }
            if mouse_button.pressed(rig_conf.mouse_drag) {
                new_transform.translation += transform.rotation * Vec3::new(event.delta.y, 0., - event.delta.x) * rig_conf.mouse_drag_sensitivity;
            }
        }

        rig_conf.transform = Some(new_transform);

        if new_transform.translation != transform.translation {
            if new_transform.translation.distance(transform.translation).abs() > 0.005 {
                transform.translation = transform.translation.lerp(new_transform.translation, time.delta().as_micros() as f32 / 100000.);
            } else {
                transform.translation = new_transform.translation;
            }
        }
        if new_transform.rotation != transform.rotation {
            if !new_transform.rotation.abs_diff_eq(transform.rotation, 0.00001) {
                transform.rotation = transform.rotation.lerp(new_transform.rotation, time.delta().as_micros() as f32 / 100000.);
            } else {
                transform.rotation = new_transform.rotation;
            }
        }
    }
}

fn strategy_camera(
    mut readers: Local<(MouseMotionEventReader, MouseWheelEventReader)>,
    time: Res<Time>,
    mouse_button: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut query: Query<(&mut Transform, &mut StrategyCamera)>
) {
    for (mut transform, mut config) in query.iter_mut() {
        if config.transform == None {
            config.transform = Some(transform.clone());
        }

        let mut new_transform = config.transform.unwrap();

        for event in readers.0.0.iter(&mouse_motion_events) {
            if mouse_button.pressed(config.mouse_rot) {
                new_transform.rotate(Quat::from_rotation_x(-config.mouse_rot_sensitivity * event.delta.y));
                new_transform.translation = Quat::from_rotation_z(-config.mouse_rot_sensitivity * event.delta.y) * new_transform.translation;
            }
        }

        for event in readers.1.0.iter(&mouse_wheel_events) {
            new_transform.translation -= new_transform.forward() * event.y * config.mouse_zoom_sensitiviy;
        }

        config.transform = Some(new_transform);

        if new_transform.translation != transform.translation {
            if new_transform.translation.distance(transform.translation).abs() > 0.005 {
                transform.translation = transform.translation.lerp(new_transform.translation, time.delta().as_micros() as f32 / 100000.);
            } else {
                transform.translation = new_transform.translation;
            }
        }
        if new_transform.rotation != transform.rotation {
            if !new_transform.rotation.abs_diff_eq(transform.rotation, 0.00001) {
                transform.rotation = transform.rotation.lerp(new_transform.rotation, time.delta().as_micros() as f32 / 100000.);
            } else {
                transform.rotation = new_transform.rotation;
            }
        }
    }
}

/// When attached to a entity the strategy camera will follow that entitiy
pub struct StrategyCameraFollow(pub bool);

fn strategy_camera_follow(
    time: Res<Time>,
    mut query_self: Query<&mut Transform, With<StrategyCameraRig>>,
    query_follow: Query<(&Transform, &StrategyCameraFollow), Changed<Transform>>
) {
    for (follow_transform, follow) in query_follow.iter() {
        if follow.0 {
            for mut transform in query_self.iter_mut() {
                if follow_transform.translation != transform.translation {
                    // if follow_transform.translation.distance(transform.translation).abs() > 1. {
                    //     transform.translation = transform.translation.lerp(follow_transform.translation, time.delta().as_micros() as f32 / 50000.);
                    // } else {
                        transform.translation = follow_transform.translation;
                    // }
                }
            }
        }
    }
}

fn debug_camera_pos(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&Transform, &GlobalTransform), With<StrategyCamera>>,
) {
    for (t, g_t) in query.iter() {
        if keyboard_input.pressed(KeyCode::Space) {
            println!("g: {:?}, l: {:?}", g_t.translation, t.translation);
        }
    }
}