//! Entity update processing systems for Dojo blockchain integration.

use bevy::prelude::*;
use crate::torii::{events::*, types::*};

/// System for processing position updates
pub fn process_position_update_events(
    mut ev_position: EventReader<PositionUpdatedEvent>,
) {
    for event in ev_position.read() {
        let position = &event.0;
        info!("Position updated: player={:?}, x={}, y={}", position.player, position.x, position.y);
        // Add your position-specific logic here
    }
}

/// System for processing moon rocks updates
pub fn process_moon_rocks_update_events(
    mut ev_moon_rocks: EventReader<MoonRocksUpdatedEvent>,
) {
    for event in ev_moon_rocks.read() {
        let moon_rocks = &event.0;
        info!("Moon rocks updated: player={:?}, amount={}", moon_rocks.player, moon_rocks.amount);
        // Add your moon rocks-specific logic here
    }
}

/// System for processing game updates
pub fn process_game_update_events(
    mut ev_game: EventReader<GameUpdatedEvent>,
) {
    for event in ev_game.read() {
        let game = &event.0;
        info!("Game updated: player={:?}, game_id={}, health={}, points={}", 
              game.player, game.game_id, game.health, game.points);
        // Add your game-specific logic here
    }
}

/// System for processing game counter updates
pub fn process_game_counter_update_events(
    mut ev_game_counter: EventReader<GameCounterUpdatedEvent>,
) {
    for event in ev_game_counter.read() {
        let counter = &event.0;
        info!("Game counter updated: player={:?}, next_game_id={}", 
              counter.player, counter.next_game_id);
        // Add your game counter-specific logic here
    }
}

/// System for processing active game updates
pub fn process_active_game_update_events(
    mut ev_active_game: EventReader<ActiveGameUpdatedEvent>,
) {
    for event in ev_active_game.read() {
        let active_game = &event.0;
        info!("Active game updated: player={:?}, game_id={}", 
              active_game.player, active_game.game_id);
        // Add your active game-specific logic here
    }
}

/// System for processing orb bag slot updates
pub fn process_orb_bag_slot_update_events(
    mut ev_orb_bag_slot: EventReader<OrbBagSlotUpdatedEvent>,
) {
    for event in ev_orb_bag_slot.read() {
        let slot = &event.0;
        info!("Orb bag slot updated: player={:?}, game_id={}, slot_index={}, orb_type={:?}, active={}", 
              slot.player, slot.game_id, slot.slot_index, slot.orb_type, slot.is_active);
        // Add your orb bag slot-specific logic here
    }
}

/// System for processing drawn orb updates
pub fn process_drawn_orb_update_events(
    mut ev_drawn_orb: EventReader<DrawnOrbUpdatedEvent>,
) {
    for event in ev_drawn_orb.read() {
        let drawn_orb = &event.0;
        info!("Drawn orb updated: player={:?}, game_id={}, draw_index={}, orb_type={:?}", 
              drawn_orb.player, drawn_orb.game_id, drawn_orb.draw_index, drawn_orb.orb_type);
        // Add your drawn orb-specific logic here
    }
}

/// System for processing shop inventory updates
pub fn process_shop_inventory_update_events(
    mut ev_shop_inventory: EventReader<ShopInventoryUpdatedEvent>,
) {
    for event in ev_shop_inventory.read() {
        let inventory = &event.0;
        info!("Shop inventory updated: player={:?}, game_id={}, level={}, slot_index={}, orb_type={:?}, price={}, rarity={:?}", 
              inventory.player, inventory.game_id, inventory.level, inventory.slot_index, 
              inventory.orb_type, inventory.base_price, inventory.rarity);
        // Add your shop inventory-specific logic here
    }
}

/// System for processing purchase history updates
pub fn process_purchase_history_update_events(
    mut ev_purchase_history: EventReader<PurchaseHistoryUpdatedEvent>,
) {
    for event in ev_purchase_history.read() {
        let history = &event.0;
        info!("Purchase history updated: player={:?}, game_id={}, orb_type={:?}, count={}", 
              history.player, history.game_id, history.orb_type, history.purchase_count);
        // Add your purchase history-specific logic here
    }
}

/// System for updating cube positions based on position events
pub fn update_cube_position(
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
    y: u32
) {
    for (mut transform, cube) in query.iter_mut() {
        if cube.player == player {
            info!("Updating cube position: ({}, {})", x, y);
            transform.translation = Vec3::new(x as f32, y as f32, 0.0);
        }
    }
}