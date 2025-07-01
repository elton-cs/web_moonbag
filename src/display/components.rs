//! UI component definitions for Dojo display system

use crate::torii::types::*;
use bevy::prelude::*;

/// Resource to hold current display data
#[derive(Resource, Default)]
pub struct DisplayData {
    pub current_game: Option<Game>,
    pub moon_rocks: Option<MoonRocks>,
    pub orb_bag_slots: Vec<OrbBagSlot>,
    pub shop_items: Vec<ShopInventory>,
    pub drawn_orbs: Vec<DrawnOrb>,
}

/// Component for the main UI root container
#[derive(Component)]
pub struct DojoDisplayRoot;

/// Component for the game stats display panel
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

/// Component for moon rocks counter
#[derive(Component)]
pub struct MoonRocksDisplay;

/// Component for game state indicator
#[derive(Component)]
pub struct GameStateDisplay;

/// Component for position display
#[derive(Component)]
pub struct PositionDisplay;

/// Component for drawn orbs history
#[derive(Component)]
pub struct DrawnOrbsPanel;
