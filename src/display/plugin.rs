//! Dojo Display Plugin for rendering blockchain game data using modern Bevy 0.16 syntax

use bevy::prelude::*;

use crate::torii::events::*;
use super::{components::*, systems::*};

pub struct DojoDisplayPlugin;

impl Plugin for DojoDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register all blockchain events
            .add_event::<GameUpdatedEvent>()
            .add_event::<MoonRocksUpdatedEvent>()
            .add_event::<PositionUpdatedEvent>()
            .add_event::<OrbBagSlotUpdatedEvent>()
            .add_event::<ShopInventoryUpdatedEvent>()
            .add_event::<DrawnOrbAddedEvent>()
            .add_event::<GameCounterUpdatedEvent>()
            .add_event::<ActiveGameUpdatedEvent>()
            .add_event::<DrawnOrbUpdatedEvent>()
            .add_event::<PurchaseHistoryUpdatedEvent>()
            
            // Initialize resources
            .init_resource::<DisplayData>()
            
            // Setup systems
            .add_systems(Startup, setup_display_ui)
            .add_systems(Update, (
                update_game_stats_display,
                update_moon_rocks_display,
                update_position_display,
                update_game_state_display,
                update_orb_bag_display,
                update_shop_display,
                update_drawn_orbs_display,
                update_game_counter_display,
                update_active_game_display,
            ));
    }
}