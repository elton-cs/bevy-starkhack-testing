use std::default;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_starkhack_testing::{handling::get_current_position, torii_client::run_torii_client};
use starknet_ff::FieldElement;
use tokio::runtime::Builder;
use torii_grpc::types::schema::Entity as ToriiEntity;

#[derive(Component)]
struct Camera;

#[derive(Resource)]
struct ToriiResource {
    entity: ToriiEntity,
    rx: tokio::sync::mpsc::Receiver<ToriiEntity>,
}

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
    last_direction: bool,
}

fn main() {
    // // run torii client in separate thread via tokio
    // let tokio_runtime = Builder::new_current_thread()
    //     .worker_threads(1)
    //     .enable_all()
    //     .build()
    //     .unwrap();

    // let (tx, rx) = tokio::sync::mpsc::channel::<Entity>(16);

    // std::thread::spawn(move || {
    //     tokio_runtime.block_on(run_torii_client(tx));
    // });

    App::new()
        .add_systems(Startup, (spawn_camera, spawn_player, setup_tokio_and_torii))
        .add_systems(
            Update,
            (update_position, print_torii_entity_updates, print_position),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_tokio_and_torii(mut commands: Commands) {
    // run torii client in separate thread via tokio
    let tokio_runtime = Builder::new_current_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let (tx, rx) = tokio::sync::mpsc::channel::<ToriiEntity>(16);

    std::thread::spawn(move || {
        tokio_runtime.block_on(run_torii_client(tx));
    });

    let default_entity = ToriiEntity {
        hashed_keys: FieldElement::default(),
        models: Vec::new(),
    };

    commands.insert_resource(ToriiResource {
        entity: default_entity,
        rx,
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera));
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
            last_direction: true,
        },
        MaterialMesh2dBundle {
            mesh: circle,
            material: materials.add(color),
            ..default()
        },
    ));
}

// fn update_torii_entity(
//     mut commands: Commands,
//     mut torii_entity: ResMut<ToriiEntity>,
//     entity: ToriiEntity,
// ) {
//     torii_entity = entity;
// }

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
            if moves.last_direction {
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
            moves.last_direction = !moves.last_direction
        }
    }
}

fn _print_position(query: Query<(Entity, &Position, &PreviousPosition)>) {
    for (entity, position, previous_position) in query.iter() {
        if previous_position.x != position.x {
            info!("Entity: {:?}, Position: {:?}", entity, position);
        }
    }
}

fn print_torii_entity_updates(mut torii_entity: ResMut<ToriiResource>) {
    if let Ok(entity) = torii_entity.rx.try_recv() {
        info!("Message from Torii Client: {:?}", entity);
        torii_entity.entity = entity;
    }
}

fn print_position(torii_entity: Res<ToriiResource>) {
    if torii_entity.entity.models.is_empty() {
        return;
    }
    let (x, y) = get_current_position(&torii_entity.entity);
    info!("Torii Entity Position: ({}, {})", x, y);
}
