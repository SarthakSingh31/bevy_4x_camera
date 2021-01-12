use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_mod_picking::{PickingPlugin, InteractablePickingPlugin, PickSource};

mod board;
mod camera;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(board::BoardPlugin)
        .add_plugin(camera::FourXCameraPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_startup_system(dev_env.system())
        .run();
}

fn dev_env(
    commands: &mut Commands
) {
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
            ..Default::default()
        })
        .spawn(camera::CameraRigBundle::default())
        // camera
        .with_children(|cb| {
            cb.spawn(
                Camera3dBundle {
                perspective_projection: PerspectiveProjection {
                    fov: 0.1,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(-20.0, 20., 0.0))
                    .looking_at(Vec3::zero(), Vec3::unit_y()),
                ..Default::default()
            })
            .with(PickSource::default());
        });
}