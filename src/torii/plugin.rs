//! Main Dojo v2 plugin for Bevy integration.

use bevy::prelude::*;

use dojo_bevy_plugin::{DojoPluginV2, DojoResourceV2};

use crate::torii::{events::*, systems::*, types::*};

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
                    process_position_update_events,
                    process_moon_rocks_update_events,
                    process_game_update_events,
                    process_game_counter_update_events,
                    process_active_game_update_events,
                    process_orb_bag_slot_update_events,
                    process_drawn_orb_update_events,
                    process_shop_inventory_update_events,
                    process_purchase_history_update_events,
                ),
            );
    }
}
