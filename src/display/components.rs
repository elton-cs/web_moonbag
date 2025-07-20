//! UI component definitions for comprehensive Dojo event display system

use crate::torii::types::*;
use bevy::prelude::*;
use std::collections::VecDeque;

/// Resource to hold current display data and event history
#[derive(Resource, Default)]
pub struct DisplayData {
    pub current_game: Option<Game>,
    pub moon_rocks: Option<MoonRocks>,
    pub position: Option<Position>,
    pub game_counter: Option<GameCounter>,
    pub active_game: Option<ActiveGame>,
    pub orb_bag_slots: Vec<OrbBagSlot>,
    pub shop_items: Vec<ShopInventory>,
    pub drawn_orbs: Vec<DrawnOrb>,
    pub purchase_history: Vec<PurchaseHistory>,

    // Event logs for real-time display
    pub recent_events: VecDeque<EventLogEntry>,
}

/// Event log entry for displaying recent blockchain events
#[derive(Clone, Debug)]
pub struct EventLogEntry {
    pub timestamp: f64,
    pub event_type: String,
    pub description: String,
    pub player: Option<starknet::core::types::Felt>,
}

/// Component for the main UI root container
#[derive(Component)]
pub struct DojoDisplayRoot;

/// Component for event log panel showing real-time updates
#[derive(Component)]
pub struct EventLogPanel;

/// Component for event log entries
#[derive(Component)]
pub struct EventLogEntryComponent {
    pub entry_id: usize,
}

/// Component for the main data view container
#[derive(Component)]
pub struct DataViewContainer;

/// Component for game stats display panel
#[derive(Component)]
pub struct GameStatsPanel;

/// Component for individual stat displays
#[derive(Component)]
pub struct HealthDisplay;

#[derive(Component)]
pub struct PointsDisplay;

#[derive(Component)]
pub struct MultiplierDisplay;

#[derive(Component)]
pub struct LevelDisplay;

#[derive(Component)]
pub struct CheddahDisplay;

/// Component for moon rocks counter
#[derive(Component)]
pub struct MoonRocksDisplay;

/// Component for position display
#[derive(Component)]
pub struct PositionDisplay;

/// Component for game counter display
#[derive(Component)]
pub struct GameCounterDisplay;

/// Component for active game display
#[derive(Component)]
pub struct ActiveGameDisplay;

/// Component for the orb bag display panel
#[derive(Component)]
pub struct OrbBagPanel;

/// Component for individual orb bag slots
#[derive(Component)]
pub struct OrbBagSlotDisplay {
    pub slot_index: u32,
}

/// Component for the shop display panel
#[derive(Component)]
pub struct ShopPanel;

/// Component for individual shop items
#[derive(Component)]
pub struct ShopItemDisplay {
    pub slot_index: u8,
}

/// Component for purchase history panel
#[derive(Component)]
pub struct PurchaseHistoryPanel;

/// Component for individual purchase history entry
#[derive(Component)]
pub struct PurchaseHistoryEntry {
    pub orb_type: OrbType,
}

/// Component for game state indicator
#[derive(Component)]
pub struct GameStateDisplay;

/// Component for drawn orbs history panel
#[derive(Component)]
pub struct DrawnOrbsPanel;

/// Component for individual drawn orb display
#[derive(Component)]
pub struct DrawnOrbDisplay {
    pub draw_index: u32,
}

/// Component for section headers
#[derive(Component)]
pub struct SectionHeader {
    pub section_name: String,
}

/// Component for scrollable panels
#[derive(Component)]
pub struct ScrollablePanel;

/// Component for data value displays
#[derive(Component)]
pub struct DataValue {
    pub field_name: String,
}
