use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
struct Camera;

#[derive(Component, Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Component, Debug)]
struct PreviousPosition {
    x: u32,
    y: u32,
}

#[derive(Component, Debug)]
struct Moves {
    remaining: u8,
    last_direction: (bool, String),
}

fn main() {
    App::new()
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = Mesh2dHandle(meshes.add(Circle { radius: 25.0 }));
    let color = Color::hsl(360. as f32 / 1. as f32, 0.95, 0.7);

    commands.spawn((
        Position { x: 0, y: 0 },
        PreviousPosition { x: 0, y: 0 },
        Moves {
            remaining: 100,
            last_direction: (true, "Up".to_string()),
        },
        MaterialMesh2dBundle {
            mesh: circle,
            material: materials.add(color),
            ..default()
        },
    ));
}

fn update_position(
    mut query: Query<(
        &mut Position,
        &mut PreviousPosition,
        &mut Moves,
        &mut Transform,
    )>,
) {
    for (mut position, mut previous_position, mut moves, mut transform) in query.iter_mut() {
        previous_position.x = position.x;
        previous_position.y = position.y;

        if moves.remaining != 0 {
            if moves.last_direction.0 {
                position.x += 1;
                // position.y += 1;
            } else {
                position.x -= 1;
                // position.y -= 1;
            }

            transform.translation.x = position.x as f32 * 3.;
            // transform.translation.y = position.y as f32 * 3.;
            moves.remaining -= 1;
        } else {
            moves.remaining = 100;
            moves.last_direction.0 = !moves.last_direction.0
        }
    }
}

fn print_position(query: Query<(Entity, &Position, &PreviousPosition, &Moves)>) {
    for (entity, position, previous_position, moves) in query.iter() {
        if previous_position.x != position.x {
            info!(
                "Entity: {:?}, Position: {:?}, Direction: {:?}",
                entity, position, moves.last_direction.1
            );
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera));
}
