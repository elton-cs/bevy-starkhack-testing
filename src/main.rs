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
    prev_entity: ToriiEntity,
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
    App::new()
        .add_systems(Startup, (spawn_camera, spawn_player, setup_tokio_and_torii))
        .add_systems(Update, (update_torii_entity, update_torii_position).chain())
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
        entity: default_entity.clone(),
        prev_entity: default_entity,
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

fn update_torii_entity(mut torii_entity: ResMut<ToriiResource>, query: Query<&Position>) {
    if let Ok(new_entity) = torii_entity.rx.try_recv() {
        // info!("Message from Torii Client: {:?}", entity);
        torii_entity.prev_entity = torii_entity.entity.clone();
        torii_entity.entity = new_entity;

        print_torii_position(torii_entity);
        print_bevy_position(query);
    }
}

fn print_torii_position(torii_entity: ResMut<ToriiResource>) {
    if !torii_entity.entity.models.is_empty() {
        let (x, y) = get_current_position(&torii_entity.entity);
        info!("Torii Entity Position: ({}, {})", x, y);
    }
}

fn print_bevy_position(query: Query<&Position>) {
    for position in query.iter() {
        info!("Bevy Entity Position: ({}, {})", position.x, position.y);
    }
}

fn update_torii_position(
    mut query: Query<(&mut Position, &mut Transform)>,
    torii_entity: Res<ToriiResource>,
) {
    if !torii_entity.entity.models.is_empty() {
        let (x, y) = get_current_position(&torii_entity.entity);

        for (mut position, mut transform) in query.iter_mut() {
            position.x = x;
            position.y = y;
            transform.translation.x = x as f32 * 10.;
            transform.translation.y = y as f32 * 10.;
        }
    }
}
