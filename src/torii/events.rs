//! Event definitions for Dojo blockchain integration.

use crate::torii::types::*;
use bevy::prelude::*;

// ==================== EVENTS ====================

/// Event triggered when position is updated
#[derive(Event)]
pub struct PositionUpdatedEvent(pub Position);

/// Event triggered when moon rocks are updated
#[derive(Event)]
pub struct MoonRocksUpdatedEvent(pub MoonRocks);

/// Event triggered when game data is updated
#[derive(Event)]
pub struct GameUpdatedEvent(pub Game);

/// Event triggered when game counter is updated
#[derive(Event)]
pub struct GameCounterUpdatedEvent(pub GameCounter);

/// Event triggered when active game is updated
#[derive(Event)]
pub struct ActiveGameUpdatedEvent(pub ActiveGame);

/// Event triggered when orb bag slot is updated
#[derive(Event)]
pub struct OrbBagSlotUpdatedEvent(pub OrbBagSlot);

/// Event triggered when drawn orb is updated
#[derive(Event)]
pub struct DrawnOrbUpdatedEvent(pub DrawnOrb);

/// Event triggered when shop inventory is updated
#[derive(Event)]
pub struct ShopInventoryUpdatedEvent(pub ShopInventory);

/// Event triggered when purchase history is updated
#[derive(Event)]
pub struct PurchaseHistoryUpdatedEvent(pub PurchaseHistory);

/// Event triggered when a new orb is drawn (for display history)
#[derive(Event)]
pub struct DrawnOrbAddedEvent(pub DrawnOrb);
