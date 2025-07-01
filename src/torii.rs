//! Dojo v2 plugin integration for Bevy with blockchain interaction.
//!
//! This module provides a plugin for integrating Dojo v2 functionality
//! into the Bevy game engine with Torii client and Starknet support.

use bevy::input::ButtonState;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use dojo_types::schema::Struct;
use starknet::core::types::Call;
use starknet::core::types::Felt;
use starknet::macros::selector;
use std::collections::HashSet;
use torii_grpc_client::types::{Pagination, PaginationDirection, Query as ToriiQuery};

use dojo_bevy_plugin::{DojoEntityUpdatedV2, DojoInitializedEventV2, DojoPluginV2, DojoResourceV2};

// Manifest related constants
const TORII_URL: &str = "http://localhost:8080";
const KATANA_URL: &str = "http://0.0.0.0:5050";

const WORLD_ADDRESS: Felt =
    Felt::from_hex_unchecked("0x04d9778a74d2c9e6e7e4a24cbe913998a80de217c66ee173a604d06dea5469c3");
const ACTION_ADDRESS: Felt =
    Felt::from_hex_unchecked("0x00b056c9813fdc442118bdfead6fda526e5daa5fd7d543304117ed80154ea752");
const SPAWN_SELECTOR: Felt = selector!("spawn");
const MOVE_SELECTOR: Felt = selector!("move");

/// Event triggered when position is updated
#[derive(Event)]
pub struct PositionUpdatedEvent(pub Position);

/// Component representing a player cube
#[derive(Component)]
pub struct Cube {
    pub player: Felt,
}

/// Resource for tracking existing entities
#[derive(Resource, Default)]
pub struct EntityTracker {
    existing_entities: HashSet<Felt>,
}

/// The position of the player in the game
#[derive(Component, Debug)]
pub struct Position {
    pub player: Felt,
    pub x: u32,
    pub y: u32,
}

/// Manual conversion from Dojo struct to Position
impl From<&Struct> for Position {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let x = struct_value
            .get("x")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let y = struct_value
            .get("y")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        Position { player, x, y }
    }
}

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

/// System for handling keyboard input
fn handle_keyboard_input(
    mut dojo: ResMut<DojoResourceV2>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.read() {
        let key_code = event.key_code;
        let is_pressed = event.state == ButtonState::Pressed;

        match key_code {
            KeyCode::KeyC if is_pressed => {
                info!("Connecting to Torii and Katana...");
                dojo.connect_torii(TORII_URL.to_string(), WORLD_ADDRESS);
                dojo.connect_predeployed_account(KATANA_URL.to_string(), 0);
            }
            KeyCode::Space if is_pressed => {
                info!("Spawning player...");
                let calls = vec![Call {
                    to: ACTION_ADDRESS,
                    selector: SPAWN_SELECTOR,
                    calldata: vec![],
                }];
                dojo.queue_tx(calls);
            }
            KeyCode::KeyS if is_pressed => {
                info!("Setting up Torii subscription...");
                dojo.subscribe_entities("position".to_string(), None);
            }
            KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::ArrowUp | KeyCode::ArrowDown
                if is_pressed =>
            {
                let direction = match key_code {
                    KeyCode::ArrowLeft => 0,
                    KeyCode::ArrowRight => 1,
                    KeyCode::ArrowUp => 2,
                    KeyCode::ArrowDown => 3,
                    _ => unreachable!(),
                };

                info!("Moving player in direction: {}", direction);
                let calls = vec![Call {
                    to: ACTION_ADDRESS,
                    selector: MOVE_SELECTOR,
                    calldata: vec![Felt::from(direction)],
                }];

                dojo.queue_tx(calls);
            }
            _ => continue,
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
            info!("Spawning new cube for player: {:?}", player);
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.2))),
                Cube { player },
                Transform::from_xyz(x as f32, y as f32, 0.0),
            ));

            entity_tracker.existing_entities.insert(player);
        } else {
            for (mut transform, cube) in query.iter_mut() {
                if cube.player == player {
                    info!("Updating cube position: ({}, {})", x, y);
                    transform.translation = Vec3::new(x as f32, y as f32, 0.0);
                }
            }
        }
    }
}

/// System for handling Dojo v2 events
fn on_dojo_events(
    mut dojo: ResMut<DojoResourceV2>,
    mut ev_initialized: EventReader<DojoInitializedEventV2>,
    mut ev_retrieve_entities: EventReader<DojoEntityUpdatedV2>,
    mut ev_position_updated: EventWriter<PositionUpdatedEvent>,
) {
    for _ in ev_initialized.read() {
        info!("Dojo v2 initialized.");

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

    for ev in ev_retrieve_entities.read() {
        info!(entity_id = ?ev.entity_id, "Torii v2 update");

        if ev.entity_id == Felt::ZERO {
            continue;
        }

        for m in &ev.models {
            debug!("Processing model: {:?}", &m);

            match m.name.as_str() {
                "di-Position" => {
                    ev_position_updated.write(PositionUpdatedEvent(m.into()));
                }
                name if name == "di-Moves" => {
                    // Handle moves model if needed
                }
                _ => {
                    warn!("Unhandled model: {:?}", m);
                }
            }
        }
    }
}

/// Export the plugin function for main.rs
pub fn plugin(app: &mut App) {
    app.add_plugins(DojoV2Plugin);
}
