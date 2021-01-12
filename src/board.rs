use bevy::prelude::*;
use bevy_mod_picking::{InteractableMesh, PickableMesh};
use rand::prelude::*;
use crate::camera::CameraRigFollow;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(board.system())
            .add_system(moving_car.system())
            .add_system(selectable_car.system());
    }
}

fn board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh_handles = [
        meshes.add(Mesh::from(shape::Box::new(0.8, 1., 0.8))),
        meshes.add(Mesh::from(shape::Box::new(0.8, 2., 0.8))),
        meshes.add(Mesh::from(shape::Box::new(0.8, 3., 0.8))),
        meshes.add(Mesh::from(shape::Box::new(0.8, 4., 0.8))),
    ];

    let material_handles = [
        materials.add(Color::rgb(0.92, 0.5, 0.2).into()),
        materials.add(Color::rgb(0.2, 0.81, 0.92).into()),
        materials.add(Color::rgb(0.62, 0.2, 0.92).into()),
        materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
    ];

    let points = (-25 ..= 25).filter(|i| i % 4 != 0);
    let points_clone = points.clone();
    let points = points
        .flat_map(move |i| points_clone.clone().map(move |j| (i, j)));

    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn_batch(points
            .map(move |i| {
                let mut rng = rand::thread_rng();
                PbrBundle {
                    mesh: cube_mesh_handles[rng.gen_range(0..4)].clone(),
                    material: material_handles[rng.gen_range(0..5)].clone(),
                    transform: Transform::from_translation(Vec3::new(i.0 as f32, 0.5, i.1 as f32)),
                    ..Default::default()
                }
            }))
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with(MovingCar {
            direction: Vec3::unit_x(),
            speed: 1.,
        })
        .with(CameraRigFollow(false))
        .with(PickableMesh::default())
        .with(InteractableMesh::default());
}

struct MovingCar {
    direction: Vec3,
    speed: f32,
}

fn moving_car(time: Res<Time>, mut query: Query<(&mut Transform, &mut MovingCar)>) {
    for (mut transform, mut car) in query.iter_mut() {
        if transform.translation.dot(car.direction).abs() > 24. {
            car.direction = - car.direction;
        }

        transform.translation += car.direction * (car.speed * time.delta().as_micros() as f32 / 1000000.);
    }
}

fn selectable_car(mut query: Query<(&InteractableMesh, &mut CameraRigFollow)>) {
    for (interactable, mut follow) in query.iter_mut() {
        if let Ok(bevy_mod_picking::MouseDownEvents::MouseJustPressed) = interactable.mouse_down_event(&Default::default(), MouseButton::Left) {
            follow.0 = true;
        }
    }
}