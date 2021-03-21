use bevy::{
    ecs::schedule::ReportExecutionOrderAmbiguities, prelude::*,
    render::camera::PerspectiveProjection,
};
use bevy_4x_camera::{CameraRigBundle, FourXCameraPlugin};

fn main() {
    App::build()
        .insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FourXCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(CameraRigBundle::default())
        .with_children(|cb| {
            cb.spawn(PerspectiveCameraBundle {
                perspective_projection: PerspectiveProjection {
                    fov: 0.1,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-20.0, 20., 0.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            });
        });
}
