//! Dojo v2 blockchain communication module.
//!
//! This module handles sending transactions and receiving entity updates
//! from the Dojo blockchain via Torii client.

use bevy::input::ButtonState;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use starknet::core::types::Call;
use starknet::core::types::Felt;
use torii_grpc_client::types::{Pagination, PaginationDirection, Query as ToriiQuery};

use dojo_bevy_plugin::{DojoEntityUpdatedV2, DojoInitializedEventV2, DojoPluginV2, DojoResourceV2};

use crate::types::*;

/// Dojo v2 plugin for Bevy integration
pub struct DojoV2Plugin;

impl Plugin for DojoV2Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DojoPluginV2)
            .init_resource::<DojoResourceV2>()
            .init_resource::<EntityTracker>()
            .add_event::<PositionUpdatedEvent>()
            .add_systems(
                Update,
                (
                    handle_keyboard_input,
                    on_dojo_events,
                    (update_cube_position).after(on_dojo_events),
                ),
            );
    }
}

/// System for handling keyboard input and sending blockchain transactions
fn handle_keyboard_input(
    mut dojo: ResMut<DojoResourceV2>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.read() {
        let key_code = event.key_code;
        let is_pressed = event.state == ButtonState::Pressed;

        match key_code {
            KeyCode::KeyC if is_pressed => {
                connect_to_blockchain(&mut dojo);
            }
            KeyCode::Space if is_pressed => {
                spawn_player(&mut dojo);
            }
            KeyCode::KeyS if is_pressed => {
                subscribe_to_entities(&mut dojo);
            }
            KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::ArrowUp | KeyCode::ArrowDown
                if is_pressed =>
            {
                let direction = match key_code {
                    KeyCode::ArrowLeft => Direction::Left,
                    KeyCode::ArrowRight => Direction::Right,
                    KeyCode::ArrowUp => Direction::Up,
                    KeyCode::ArrowDown => Direction::Down,
                    _ => unreachable!(),
                };
                move_player(&mut dojo, direction);
            }
            _ => continue,
        }
    }
}

/// Connect to Torii and Katana blockchain services
fn connect_to_blockchain(dojo: &mut ResMut<DojoResourceV2>) {
    info!("Connecting to Torii and Katana...");
    dojo.connect_torii(TORII_URL.to_string(), WORLD_ADDRESS);
    dojo.connect_predeployed_account(KATANA_URL.to_string(), 0);
}

/// Send spawn transaction to blockchain
fn spawn_player(dojo: &mut ResMut<DojoResourceV2>) {
    info!("Spawning player...");
    let calls = vec![Call {
        to: ACTION_ADDRESS,
        selector: SPAWN_SELECTOR,
        calldata: vec![],
    }];
    dojo.queue_tx(calls);
}

/// Subscribe to entity updates from Torii
fn subscribe_to_entities(dojo: &mut ResMut<DojoResourceV2>) {
    info!("Setting up Torii subscription...");
    dojo.subscribe_entities("position".to_string(), None);
}

/// Send move transaction to blockchain
fn move_player(dojo: &mut ResMut<DojoResourceV2>, direction: Direction) {
    info!("Moving player in direction: {:?}", direction);
    let calls = vec![Call {
        to: ACTION_ADDRESS,
        selector: MOVE_SELECTOR,
        calldata: vec![direction.into()],
    }];
    dojo.queue_tx(calls);
}

/// System for handling Dojo v2 events and entity updates
fn on_dojo_events(
    mut dojo: ResMut<DojoResourceV2>,
    mut ev_initialized: EventReader<DojoInitializedEventV2>,
    mut ev_retrieve_entities: EventReader<DojoEntityUpdatedV2>,
    mut ev_position_updated: EventWriter<PositionUpdatedEvent>,
) {
    for _ in ev_initialized.read() {
        info!("Dojo v2 initialized.");
        fetch_initial_entities(&mut dojo);
    }

    for ev in ev_retrieve_entities.read() {
        process_entity_update(ev, &mut ev_position_updated);
    }
}

/// Fetch initial entities from blockchain
fn fetch_initial_entities(dojo: &mut ResMut<DojoResourceV2>) {
    dojo.queue_retrieve_entities(ToriiQuery {
        clause: None,
        pagination: Pagination {
            limit: 100,
            cursor: None,
            direction: PaginationDirection::Forward,
            order_by: vec![],
        },
        no_hashed_keys: false,
        models: vec![],
        historical: false,
    });
}

/// Process entity updates from blockchain
fn process_entity_update(
    ev: &DojoEntityUpdatedV2,
    ev_position_updated: &mut EventWriter<PositionUpdatedEvent>,
) {
    info!(entity_id = ?ev.entity_id, "Torii v2 update");

    if ev.entity_id == Felt::ZERO {
        return;
    }

    for m in &ev.models {
        debug!("Processing model: {:?}", &m);

        match m.name.as_str() {
            "di-Position" => {
                ev_position_updated.write(PositionUpdatedEvent(m.into()));
            }
            "di-Moves" => {
                // Handle moves model if needed in the future
            }
            _ => {
                warn!("Unhandled model: {:?}", m);
            }
        }
    }
}

/// System for updating cube positions based on position events
fn update_cube_position(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entity_tracker: ResMut<EntityTracker>,
    mut ev_position_updated: EventReader<PositionUpdatedEvent>,
    mut query: Query<(&mut Transform, &Cube)>,
) {
    for ev in ev_position_updated.read() {
        let Position { x, y, player } = ev.0;

        if !entity_tracker.existing_entities.contains(&player) {
            spawn_new_cube(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut entity_tracker,
                player,
                x,
                y,
            );
        } else {
            update_existing_cube(&mut query, player, x, y);
        }
    }
}

/// Spawn a new cube for a player
fn spawn_new_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    entity_tracker: &mut ResMut<EntityTracker>,
    player: Felt,
    x: u32,
    y: u32,
) {
    info!("Spawning new cube for player: {:?}", player);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.2))),
        Cube { player },
        Transform::from_xyz(x as f32, y as f32, 0.0),
    ));

    entity_tracker.existing_entities.insert(player);
}

/// Update position of existing cube
fn update_existing_cube(query: &mut Query<(&mut Transform, &Cube)>, player: Felt, x: u32, y: u32) {
    for (mut transform, cube) in query.iter_mut() {
        if cube.player == player {
            info!("Updating cube position: ({}, {})", x, y);
            transform.translation = Vec3::new(x as f32, y as f32, 0.0);
        }
    }
}

/// Export the plugin function for lib.rs
pub fn plugin(app: &mut App) {
    app.add_plugins(DojoV2Plugin);
}
