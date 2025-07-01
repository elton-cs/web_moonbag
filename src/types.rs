//! Type definitions for the Dojo blockchain integration.
//!
//! This module contains all the types, components, events, and constants
//! used for blockchain interaction and game state management.

use bevy::prelude::*;
use dojo_types::schema::Struct;
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