//! Entity update processing systems for Dojo blockchain integration.

use bevy::input::ButtonState;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use starknet::core::types::Call;
use starknet::core::types::Felt;
use torii_grpc_client::types::{Pagination, PaginationDirection, Query as ToriiQuery};

use dojo_bevy_plugin::{DojoEntityUpdatedV2, DojoInitializedEventV2, DojoResourceV2};

use crate::torii::{constants::*, events::*, types::*};

/// System for processing moon rocks updates
pub fn process_moon_rocks_update_events(mut ev_moon_rocks: EventReader<MoonRocksUpdatedEvent>) {
    for event in ev_moon_rocks.read() {
        let moon_rocks = &event.0;
        info!(
            "Moon rocks updated: player={:?}, amount={}",
            moon_rocks.player, moon_rocks.amount
        );
        // Add your moon rocks-specific logic here
    }
}

/// System for processing game updates
pub fn process_game_update_events(mut ev_game: EventReader<GameUpdatedEvent>) {
    for event in ev_game.read() {
        let game = &event.0;
        info!(
            "Game updated: player={:?}, game_id={}, health={}, points={}",
            game.player, game.game_id, game.health, game.points
        );
        // Add your game-specific logic here
    }
}

/// System for processing game counter updates
pub fn process_game_counter_update_events(
    mut ev_game_counter: EventReader<GameCounterUpdatedEvent>,
) {
    for event in ev_game_counter.read() {
        let counter = &event.0;
        info!(
            "Game counter updated: player={:?}, next_game_id={}",
            counter.player, counter.next_game_id
        );
        // Add your game counter-specific logic here
    }
}

/// System for processing active game updates
pub fn process_active_game_update_events(mut ev_active_game: EventReader<ActiveGameUpdatedEvent>) {
    for event in ev_active_game.read() {
        let active_game = &event.0;
        info!(
            "Active game updated: player={:?}, game_id={}",
            active_game.player, active_game.game_id
        );
        // Add your active game-specific logic here
    }
}

/// System for processing orb bag slot updates
pub fn process_orb_bag_slot_update_events(
    mut ev_orb_bag_slot: EventReader<OrbBagSlotUpdatedEvent>,
) {
    for event in ev_orb_bag_slot.read() {
        let slot = &event.0;
        info!(
            "Orb bag slot updated: player={:?}, game_id={}, slot_index={}, orb_type={:?}, active={}",
            slot.player, slot.game_id, slot.slot_index, slot.orb_type, slot.is_active
        );
        // Add your orb bag slot-specific logic here
    }
}

/// System for processing drawn orb updates
pub fn process_drawn_orb_update_events(mut ev_drawn_orb: EventReader<DrawnOrbUpdatedEvent>) {
    for event in ev_drawn_orb.read() {
        let drawn_orb = &event.0;
        info!(
            "Drawn orb updated: player={:?}, game_id={}, draw_index={}, orb_type={:?}",
            drawn_orb.player, drawn_orb.game_id, drawn_orb.draw_index, drawn_orb.orb_type
        );
        // Add your drawn orb-specific logic here
    }
}

/// System for processing shop inventory updates
pub fn process_shop_inventory_update_events(
    mut ev_shop_inventory: EventReader<ShopInventoryUpdatedEvent>,
) {
    for event in ev_shop_inventory.read() {
        let inventory = &event.0;
        info!(
            "Shop inventory updated: player={:?}, game_id={}, level={}, slot_index={}, orb_type={:?}, price={}, rarity={:?}",
            inventory.player,
            inventory.game_id,
            inventory.level,
            inventory.slot_index,
            inventory.orb_type,
            inventory.base_price,
            inventory.rarity
        );
        // Add your shop inventory-specific logic here
    }
}

/// System for processing purchase history updates
pub fn process_purchase_history_update_events(
    mut ev_purchase_history: EventReader<PurchaseHistoryUpdatedEvent>,
) {
    for event in ev_purchase_history.read() {
        let history = &event.0;
        info!(
            "Purchase history updated: player={:?}, game_id={}, orb_type={:?}, count={}",
            history.player, history.game_id, history.orb_type, history.purchase_count
        );
        // Add your purchase history-specific logic here
    }
}

/// Spawn a new cube for a player
fn spawn_new_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    entity_tracker: &mut ResMut<EntityTracker>,
    player: starknet::core::types::Felt,
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
fn update_existing_cube(
    query: &mut Query<(&mut Transform, &Cube)>,
    player: starknet::core::types::Felt,
    x: u32,
    y: u32,
) {
    for (mut transform, cube) in query.iter_mut() {
        if cube.player == player {
            info!("Updating cube position: ({}, {})", x, y);
            transform.translation = Vec3::new(x as f32, y as f32, 0.0);
        }
    }
}

/// System for handling keyboard input and sending blockchain transactions
pub fn handle_keyboard_input(
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
            // KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::ArrowUp | KeyCode::ArrowDown
            //     if is_pressed =>
            // {
            //     let direction = match key_code {
            //         KeyCode::ArrowLeft => Direction::Left,
            //         KeyCode::ArrowRight => Direction::Right,
            //         KeyCode::ArrowUp => Direction::Up,
            //         KeyCode::ArrowDown => Direction::Down,
            //         _ => unreachable!(),
            //     };
            //     move_player(&mut dojo, direction);
            // }
            _ => continue,
        }
    }
}

/// System for handling Dojo v2 events and entity updates
pub fn on_dojo_events(
    mut dojo: ResMut<DojoResourceV2>,
    mut ev_initialized: EventReader<DojoInitializedEventV2>,
    mut ev_retrieve_entities: EventReader<DojoEntityUpdatedV2>,
    mut ev_moon_rocks_updated: EventWriter<MoonRocksUpdatedEvent>,
    mut ev_game_updated: EventWriter<GameUpdatedEvent>,
    mut ev_game_counter_updated: EventWriter<GameCounterUpdatedEvent>,
    mut ev_active_game_updated: EventWriter<ActiveGameUpdatedEvent>,
    mut ev_orb_bag_slot_updated: EventWriter<OrbBagSlotUpdatedEvent>,
    mut ev_drawn_orb_updated: EventWriter<DrawnOrbUpdatedEvent>,
    mut ev_shop_inventory_updated: EventWriter<ShopInventoryUpdatedEvent>,
    mut ev_purchase_history_updated: EventWriter<PurchaseHistoryUpdatedEvent>,
) {
    for _ in ev_initialized.read() {
        info!("Dojo v2 initialized.");
        fetch_initial_entities(&mut dojo);
    }

    for ev in ev_retrieve_entities.read() {
        process_entity_update(
            ev,
            &mut ev_moon_rocks_updated,
            &mut ev_game_updated,
            &mut ev_game_counter_updated,
            &mut ev_active_game_updated,
            &mut ev_orb_bag_slot_updated,
            &mut ev_drawn_orb_updated,
            &mut ev_shop_inventory_updated,
            &mut ev_purchase_history_updated,
        );
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
    ev_moon_rocks_updated: &mut EventWriter<MoonRocksUpdatedEvent>,
    ev_game_updated: &mut EventWriter<GameUpdatedEvent>,
    ev_game_counter_updated: &mut EventWriter<GameCounterUpdatedEvent>,
    ev_active_game_updated: &mut EventWriter<ActiveGameUpdatedEvent>,
    ev_orb_bag_slot_updated: &mut EventWriter<OrbBagSlotUpdatedEvent>,
    ev_drawn_orb_updated: &mut EventWriter<DrawnOrbUpdatedEvent>,
    ev_shop_inventory_updated: &mut EventWriter<ShopInventoryUpdatedEvent>,
    ev_purchase_history_updated: &mut EventWriter<PurchaseHistoryUpdatedEvent>,
) {
    info!(entity_id = ?ev.entity_id, "Torii v2 update");

    if ev.entity_id == Felt::ZERO {
        return;
    }

    for m in &ev.models {
        debug!("Processing model: {:?}", &m);

        match m.name.as_str() {
            "di-MoonRocks" => {
                ev_moon_rocks_updated.write(MoonRocksUpdatedEvent(m.into()));
            }
            "di-Game" => {
                ev_game_updated.write(GameUpdatedEvent(m.into()));
            }
            "di-GameCounter" => {
                ev_game_counter_updated.write(GameCounterUpdatedEvent(m.into()));
            }
            "di-ActiveGame" => {
                ev_active_game_updated.write(ActiveGameUpdatedEvent(m.into()));
            }
            "di-OrbBagSlot" => {
                ev_orb_bag_slot_updated.write(OrbBagSlotUpdatedEvent(m.into()));
            }
            "di-DrawnOrb" => {
                ev_drawn_orb_updated.write(DrawnOrbUpdatedEvent(m.into()));
            }
            "di-ShopInventory" => {
                ev_shop_inventory_updated.write(ShopInventoryUpdatedEvent(m.into()));
            }
            "di-PurchaseHistory" => {
                ev_purchase_history_updated.write(PurchaseHistoryUpdatedEvent(m.into()));
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
