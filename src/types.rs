//! Type definitions for the Dojo blockchain integration.
//!
//! This module contains all the types, components, events, and constants
//! used for blockchain interaction and game state management.

use bevy::prelude::*;
use dojo_types::schema::{Enum, Struct};
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;
use starknet::macros::selector;
use std::collections::HashSet;

// Manifest related constants
pub const TORII_URL: &str = "http://localhost:8080";
pub const KATANA_URL: &str = "http://0.0.0.0:5050";

pub const WORLD_ADDRESS: Felt =
    Felt::from_hex_unchecked("0x04d9778a74d2c9e6e7e4a24cbe913998a80de217c66ee173a604d06dea5469c3");
pub const ACTION_ADDRESS: Felt =
    Felt::from_hex_unchecked("0x00b056c9813fdc442118bdfead6fda526e5daa5fd7d543304117ed80154ea752");
pub const SPAWN_SELECTOR: Felt = selector!("spawn");
pub const MOVE_SELECTOR: Felt = selector!("move");

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

/// Component representing a player cube
#[derive(Component)]
pub struct Cube {
    pub player: Felt,
}

/// Resource for tracking existing entities
#[derive(Resource, Default)]
pub struct EntityTracker {
    pub existing_entities: HashSet<Felt>,
}

/// The position of the player in the game
#[derive(Component, Debug, Clone, Copy)]
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

/// Direction enum for movement
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

impl From<Direction> for Felt {
    fn from(direction: Direction) -> Self {
        Felt::from(direction as u8)
    }
}

/// Conversion from Dojo enum to GameState
impl From<&dojo_types::schema::Enum> for GameState {
    fn from(enum_value: &dojo_types::schema::Enum) -> Self {
        let state_value = enum_value.option.unwrap_or(0);
        
        match state_value {
            0 => GameState::Active,
            1 => GameState::LevelComplete,
            2 => GameState::GameWon,
            3 => GameState::GameLost,
            _ => panic!("Invalid GameState value: {}", state_value),
        }
    }
}

/// Conversion from Dojo enum to ShopRarity
impl From<&Enum> for ShopRarity {
    fn from(enum_value: &Enum) -> Self {
        let rarity_value = enum_value.option.unwrap_or(0);
        
        match rarity_value {
            0 => ShopRarity::Common,
            1 => ShopRarity::Rare,
            2 => ShopRarity::Cosmic,
            _ => panic!("Invalid ShopRarity value: {}", rarity_value),
        }
    }
}

/// Conversion from Dojo enum to OrbType
impl From<&Enum> for OrbType {
    fn from(enum_value: &Enum) -> Self {
        let orb_value = enum_value.option.unwrap_or(0);
        
        match orb_value {
            0 => OrbType::SingleBomb,
            1 => OrbType::DoubleBomb,
            2 => OrbType::TripleBomb,
            3 => OrbType::FivePoints,
            4 => OrbType::DoubleMultiplier,
            5 => OrbType::RemainingOrbs,
            6 => OrbType::BombCounter,
            7 => OrbType::Health,
            8 => OrbType::CheddahBomb,
            9 => OrbType::SevenPoints,
            10 => OrbType::MoonRock,
            11 => OrbType::HalfMultiplier,
            12 => OrbType::EightPoints,
            13 => OrbType::NinePoints,
            14 => OrbType::NextPoints2x,
            15 => OrbType::Multiplier1_5x,
            16 => OrbType::BigHealth,
            17 => OrbType::BigMoonRock,
            _ => panic!("Invalid OrbType value: {}", orb_value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoonRocks {
    pub player: Felt,
    pub amount: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    pub player: Felt,
    pub game_id: u32,
    pub health: u8,
    pub points: u32,
    pub multiplier: u32,
    pub cheddah: u32,
    pub current_level: u8,
    pub is_active: bool,
    pub game_state: GameState,
    pub orb_bag_size: u32,
    pub orbs_drawn_count: u32,
    pub bombs_drawn_count: u32,
    pub temp_multiplier_active: bool,
    pub temp_multiplier_value: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameCounter {
    pub player: Felt,
    pub next_game_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveGame {
    pub player: Felt,
    pub game_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum OrbType {
    SingleBomb,
    DoubleBomb,
    TripleBomb,
    FivePoints,
    DoubleMultiplier,
    RemainingOrbs,
    BombCounter,
    Health,
    CheddahBomb,
    SevenPoints,
    MoonRock,
    HalfMultiplier,
    EightPoints,
    NinePoints,
    NextPoints2x,
    Multiplier1_5x,
    BigHealth,
    BigMoonRock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Active,
    LevelComplete,
    GameWon,
    GameLost,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrbBagSlot {
    pub player: Felt,
    pub game_id: u32,
    pub slot_index: u32,
    pub orb_type: OrbType,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrawnOrb {
    pub player: Felt,
    pub game_id: u32,
    pub draw_index: u32,
    pub orb_type: OrbType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShopRarity {
    Common,
    Rare,
    Cosmic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShopInventory {
    pub player: Felt,
    pub game_id: u32,
    pub level: u8,
    pub slot_index: u8,
    pub orb_type: OrbType,
    pub base_price: u32,
    pub rarity: ShopRarity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurchaseHistory {
    pub player: Felt,
    pub game_id: u32,
    pub orb_type: OrbType,
    pub purchase_count: u32,
}

/// Conversion from Dojo struct to MoonRocks
impl From<&Struct> for MoonRocks {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let amount = struct_value
            .get("amount")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        MoonRocks { player, amount }
    }
}

/// Conversion from Dojo struct to GameCounter
impl From<&Struct> for GameCounter {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let next_game_id = struct_value
            .get("next_game_id")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        GameCounter { player, next_game_id }
    }
}

/// Conversion from Dojo struct to ActiveGame
impl From<&Struct> for ActiveGame {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let game_id = struct_value
            .get("game_id")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        ActiveGame { player, game_id }
    }
}

/// Conversion from Dojo struct to Game
impl From<&Struct> for Game {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let game_id = struct_value
            .get("game_id")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let health = struct_value
            .get("health")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let points = struct_value
            .get("points")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let multiplier = struct_value
            .get("multiplier")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let cheddah = struct_value
            .get("cheddah")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let current_level = struct_value
            .get("current_level")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let is_active = struct_value
            .get("is_active")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_bool()
            .unwrap();
        let game_state = GameState::from(struct_value.get("game_state").unwrap().as_enum().unwrap());
        let orb_bag_size = struct_value
            .get("orb_bag_size")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let orbs_drawn_count = struct_value
            .get("orbs_drawn_count")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let bombs_drawn_count = struct_value
            .get("bombs_drawn_count")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let temp_multiplier_active = struct_value
            .get("temp_multiplier_active")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_bool()
            .unwrap();
        let temp_multiplier_value = struct_value
            .get("temp_multiplier_value")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        Game {
            player,
            game_id,
            health,
            points,
            multiplier,
            cheddah,
            current_level,
            is_active,
            game_state,
            orb_bag_size,
            orbs_drawn_count,
            bombs_drawn_count,
            temp_multiplier_active,
            temp_multiplier_value,
        }
    }
}

/// Conversion from Dojo struct to OrbBagSlot
impl From<&Struct> for OrbBagSlot {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let game_id = struct_value
            .get("game_id")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let slot_index = struct_value
            .get("slot_index")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let orb_type = OrbType::from(struct_value.get("orb_type").unwrap().as_enum().unwrap());
        let is_active = struct_value
            .get("is_active")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_bool()
            .unwrap();

        OrbBagSlot {
            player,
            game_id,
            slot_index,
            orb_type,
            is_active,
        }
    }
}

/// Conversion from Dojo struct to DrawnOrb
impl From<&Struct> for DrawnOrb {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let game_id = struct_value
            .get("game_id")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let draw_index = struct_value
            .get("draw_index")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let orb_type = OrbType::from(struct_value.get("orb_type").unwrap().as_enum().unwrap());

        DrawnOrb {
            player,
            game_id,
            draw_index,
            orb_type,
        }
    }
}

/// Conversion from Dojo struct to ShopInventory
impl From<&Struct> for ShopInventory {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let game_id = struct_value
            .get("game_id")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let level = struct_value
            .get("level")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let slot_index = struct_value
            .get("slot_index")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap();
        let orb_type = OrbType::from(struct_value.get("orb_type").unwrap().as_enum().unwrap());
        let base_price = struct_value
            .get("base_price")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let rarity = ShopRarity::from(struct_value.get("rarity").unwrap().as_enum().unwrap());

        ShopInventory {
            player,
            game_id,
            level,
            slot_index,
            orb_type,
            base_price,
            rarity,
        }
    }
}

/// Conversion from Dojo struct to PurchaseHistory
impl From<&Struct> for PurchaseHistory {
    fn from(struct_value: &Struct) -> Self {
        let player = struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap();
        let game_id = struct_value
            .get("game_id")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();
        let orb_type = OrbType::from(struct_value.get("orb_type").unwrap().as_enum().unwrap());
        let purchase_count = struct_value
            .get("purchase_count")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap();

        PurchaseHistory {
            player,
            game_id,
            orb_type,
            purchase_count,
        }
    }
}
