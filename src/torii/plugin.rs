//! Main Dojo v2 plugin for Bevy integration.

use bevy::input::ButtonState;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use starknet::core::types::Call;
use starknet::core::types::Felt;
use torii_grpc_client::types::{Pagination, PaginationDirection, Query as ToriiQuery};

use dojo_bevy_plugin::{DojoEntityUpdatedV2, DojoInitializedEventV2, DojoPluginV2, DojoResourceV2};

use crate::torii::{constants::*, events::*, systems::*, types::*};

/// Dojo v2 plugin for Bevy integration
pub struct DojoV2Plugin;

impl Plugin for DojoV2Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DojoPluginV2)
            .init_resource::<DojoResourceV2>()
            .init_resource::<EntityTracker>()
            .add_event::<PositionUpdatedEvent>()
            .add_event::<MoonRocksUpdatedEvent>()
            .add_event::<GameUpdatedEvent>()
            .add_event::<GameCounterUpdatedEvent>()
            .add_event::<ActiveGameUpdatedEvent>()
            .add_event::<OrbBagSlotUpdatedEvent>()
            .add_event::<DrawnOrbUpdatedEvent>()
            .add_event::<ShopInventoryUpdatedEvent>()
            .add_event::<PurchaseHistoryUpdatedEvent>()
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
            &mut ev_position_updated,
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
            "di-Position" => {
                ev_position_updated.write(PositionUpdatedEvent(m.into()));
            }
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