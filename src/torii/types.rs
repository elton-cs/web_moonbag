//! Type definitions for Dojo blockchain integration.

use bevy::prelude::*;
use dojo_types::schema::{Enum, Struct};
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;
use std::collections::HashSet;

// ==================== COMPONENTS ====================

/// Component representing a player cube
#[derive(Component)]
pub struct Cube {
    pub player: Felt,
}

// ==================== RESOURCES ====================

/// Resource for tracking existing entities
#[derive(Resource, Default)]
pub struct EntityTracker {
    pub existing_entities: HashSet<Felt>,
}

// ==================== GAME TYPES ====================

/// The position of the player in the game
#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub player: Felt,
    pub x: u32,
    pub y: u32,
}

/// Direction enum for movement
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
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

// ==================== ENUMS ====================

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
pub enum ShopRarity {
    Common,
    Rare,
    Cosmic,
}

// ==================== CONVERSION HELPERS ====================

mod conversion_helpers {
    use super::*;

    /// Extract player field from Dojo struct
    pub fn extract_player(struct_value: &Struct) -> Felt {
        struct_value
            .get("player")
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_contract_address()
            .unwrap()
    }

    /// Extract u32 field from Dojo struct
    pub fn extract_u32(struct_value: &Struct, field: &str) -> u32 {
        struct_value
            .get(field)
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap()
    }

    /// Extract u8 field from Dojo struct
    pub fn extract_u8(struct_value: &Struct, field: &str) -> u8 {
        struct_value
            .get(field)
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap()
    }

    /// Extract bool field from Dojo struct
    pub fn extract_bool(struct_value: &Struct, field: &str) -> bool {
        struct_value
            .get(field)
            .unwrap()
            .as_primitive()
            .unwrap()
            .as_bool()
            .unwrap()
    }

    /// Extract and convert enum field from Dojo struct
    pub fn extract_enum<T>(struct_value: &Struct, field: &str) -> T
    where
        T: for<'a> From<&'a Enum>,
    {
        T::from(struct_value.get(field).unwrap().as_enum().unwrap())
    }

    /// Generic enum to variant conversion with error handling
    pub fn enum_to_variant<T: Clone>(
        enum_value: &Enum,
        variants: &[(u8, T)],
        type_name: &str,
    ) -> T {
        let value = enum_value.option.unwrap_or(0);

        for (variant_value, variant) in variants {
            if *variant_value == value {
                return variant.clone();
            }
        }

        panic!("Invalid {} value: {}", type_name, value)
    }
}

use conversion_helpers::*;

// ==================== CONVERSIONS ====================

impl From<Direction> for Felt {
    fn from(direction: Direction) -> Self {
        Felt::from(direction as u8)
    }
}

/// Conversion from Dojo enum to GameState
impl From<&dojo_types::schema::Enum> for GameState {
    fn from(enum_value: &dojo_types::schema::Enum) -> Self {
        enum_to_variant(
            enum_value,
            &[
                (0, GameState::Active),
                (1, GameState::LevelComplete),
                (2, GameState::GameWon),
                (3, GameState::GameLost),
            ],
            "GameState",
        )
    }
}

/// Conversion from Dojo enum to ShopRarity
impl From<&Enum> for ShopRarity {
    fn from(enum_value: &Enum) -> Self {
        enum_to_variant(
            enum_value,
            &[
                (0, ShopRarity::Common),
                (1, ShopRarity::Rare),
                (2, ShopRarity::Cosmic),
            ],
            "ShopRarity",
        )
    }
}

/// Conversion from Dojo enum to OrbType
impl From<&Enum> for OrbType {
    fn from(enum_value: &Enum) -> Self {
        enum_to_variant(
            enum_value,
            &[
                (0, OrbType::SingleBomb),
                (1, OrbType::DoubleBomb),
                (2, OrbType::TripleBomb),
                (3, OrbType::FivePoints),
                (4, OrbType::DoubleMultiplier),
                (5, OrbType::RemainingOrbs),
                (6, OrbType::BombCounter),
                (7, OrbType::Health),
                (8, OrbType::CheddahBomb),
                (9, OrbType::SevenPoints),
                (10, OrbType::MoonRock),
                (11, OrbType::HalfMultiplier),
                (12, OrbType::EightPoints),
                (13, OrbType::NinePoints),
                (14, OrbType::NextPoints2x),
                (15, OrbType::Multiplier1_5x),
                (16, OrbType::BigHealth),
                (17, OrbType::BigMoonRock),
            ],
            "OrbType",
        )
    }
}

/// Conversion from Dojo struct to Position
impl From<&Struct> for Position {
    fn from(struct_value: &Struct) -> Self {
        Position {
            player: extract_player(struct_value),
            x: extract_u32(struct_value, "x"),
            y: extract_u32(struct_value, "y"),
        }
    }
}

/// Conversion from Dojo struct to MoonRocks
impl From<&Struct> for MoonRocks {
    fn from(struct_value: &Struct) -> Self {
        MoonRocks {
            player: extract_player(struct_value),
            amount: extract_u32(struct_value, "amount"),
        }
    }
}

/// Conversion from Dojo struct to GameCounter
impl From<&Struct> for GameCounter {
    fn from(struct_value: &Struct) -> Self {
        GameCounter {
            player: extract_player(struct_value),
            next_game_id: extract_u32(struct_value, "next_game_id"),
        }
    }
}

/// Conversion from Dojo struct to ActiveGame
impl From<&Struct> for ActiveGame {
    fn from(struct_value: &Struct) -> Self {
        ActiveGame {
            player: extract_player(struct_value),
            game_id: extract_u32(struct_value, "game_id"),
        }
    }
}

/// Conversion from Dojo struct to Game
impl From<&Struct> for Game {
    fn from(struct_value: &Struct) -> Self {
        Game {
            player: extract_player(struct_value),
            game_id: extract_u32(struct_value, "game_id"),
            health: extract_u8(struct_value, "health"),
            points: extract_u32(struct_value, "points"),
            multiplier: extract_u32(struct_value, "multiplier"),
            cheddah: extract_u32(struct_value, "cheddah"),
            current_level: extract_u8(struct_value, "current_level"),
            is_active: extract_bool(struct_value, "is_active"),
            game_state: extract_enum(struct_value, "game_state"),
            orb_bag_size: extract_u32(struct_value, "orb_bag_size"),
            orbs_drawn_count: extract_u32(struct_value, "orbs_drawn_count"),
            bombs_drawn_count: extract_u32(struct_value, "bombs_drawn_count"),
            temp_multiplier_active: extract_bool(struct_value, "temp_multiplier_active"),
            temp_multiplier_value: extract_u32(struct_value, "temp_multiplier_value"),
        }
    }
}

/// Conversion from Dojo struct to OrbBagSlot
impl From<&Struct> for OrbBagSlot {
    fn from(struct_value: &Struct) -> Self {
        OrbBagSlot {
            player: extract_player(struct_value),
            game_id: extract_u32(struct_value, "game_id"),
            slot_index: extract_u32(struct_value, "slot_index"),
            orb_type: extract_enum(struct_value, "orb_type"),
            is_active: extract_bool(struct_value, "is_active"),
        }
    }
}

/// Conversion from Dojo struct to DrawnOrb
impl From<&Struct> for DrawnOrb {
    fn from(struct_value: &Struct) -> Self {
        DrawnOrb {
            player: extract_player(struct_value),
            game_id: extract_u32(struct_value, "game_id"),
            draw_index: extract_u32(struct_value, "draw_index"),
            orb_type: extract_enum(struct_value, "orb_type"),
        }
    }
}

/// Conversion from Dojo struct to ShopInventory
impl From<&Struct> for ShopInventory {
    fn from(struct_value: &Struct) -> Self {
        ShopInventory {
            player: extract_player(struct_value),
            game_id: extract_u32(struct_value, "game_id"),
            level: extract_u8(struct_value, "level"),
            slot_index: extract_u8(struct_value, "slot_index"),
            orb_type: extract_enum(struct_value, "orb_type"),
            base_price: extract_u32(struct_value, "base_price"),
            rarity: extract_enum(struct_value, "rarity"),
        }
    }
}

/// Conversion from Dojo struct to PurchaseHistory
impl From<&Struct> for PurchaseHistory {
    fn from(struct_value: &Struct) -> Self {
        PurchaseHistory {
            player: extract_player(struct_value),
            game_id: extract_u32(struct_value, "game_id"),
            orb_type: extract_enum(struct_value, "orb_type"),
            purchase_count: extract_u32(struct_value, "purchase_count"),
        }
    }
}
