use dojo_types::primitive::Primitive;
use dojo_types::schema::{Struct, Ty};
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

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

#[derive(Debug)]
pub enum ConversionError {
    UnknownModelType(String),
    MissingField(String),
    InvalidFieldType(String),
    ConversionFailed(String),
}

#[derive(Debug, Clone)]
pub enum DojoModel {
    MoonRocks(MoonRocks),
    Game(Game),
    GameCounter(GameCounter),
    ActiveGame(ActiveGame),
    OrbBagSlot(OrbBagSlot),
    DrawnOrb(DrawnOrb),
    ShopInventory(ShopInventory),
    PurchaseHistory(PurchaseHistory),
}

pub fn convert_dojo_struct(dojo_struct: &Struct) -> Result<DojoModel, ConversionError> {
    match dojo_struct.name.as_str() {
        "di-MoonRocks" => {
            let moon_rocks = MoonRocks::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::MoonRocks(moon_rocks))
        }
        "di-Game" => {
            let game = Game::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::Game(game))
        }
        "di-GameCounter" => {
            let game_counter = GameCounter::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::GameCounter(game_counter))
        }
        "di-ActiveGame" => {
            let active_game = ActiveGame::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::ActiveGame(active_game))
        }
        "di-OrbBagSlot" => {
            let orb_bag_slot = OrbBagSlot::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::OrbBagSlot(orb_bag_slot))
        }
        "di-DrawnOrb" => {
            let drawn_orb = DrawnOrb::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::DrawnOrb(drawn_orb))
        }
        "di-ShopInventory" => {
            let shop_inventory = ShopInventory::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::ShopInventory(shop_inventory))
        }
        "di-PurchaseHistory" => {
            let purchase_history = PurchaseHistory::from_dojo_struct(dojo_struct)?;
            Ok(DojoModel::PurchaseHistory(purchase_history))
        }
        _ => Err(ConversionError::UnknownModelType(dojo_struct.name.clone())),
    }
}

fn get_field_value<T>(
    dojo_struct: &Struct,
    field_name: &str,
    converter: fn(&Ty) -> Result<T, ConversionError>,
) -> Result<T, ConversionError> {
    let member = dojo_struct
        .children
        .iter()
        .find(|member| member.name == field_name)
        .ok_or_else(|| ConversionError::MissingField(field_name.to_string()))?;

    converter(&member.ty)
}

fn felt_from_ty(ty: &Ty) -> Result<Felt, ConversionError> {
    match ty {
        Ty::Primitive(Primitive::ContractAddress(Some(felt))) => Ok(*felt),
        Ty::Primitive(Primitive::ContractAddress(None)) => Err(ConversionError::InvalidFieldType(
            "ContractAddress is None".to_string(),
        )),
        _ => Err(ConversionError::InvalidFieldType(format!(
            "Expected ContractAddress, got {:?}",
            ty
        ))),
    }
}

fn u32_from_ty(ty: &Ty) -> Result<u32, ConversionError> {
    match ty {
        Ty::Primitive(Primitive::U32(Some(value))) => Ok(*value),
        Ty::Primitive(Primitive::U32(None)) => {
            Err(ConversionError::InvalidFieldType("U32 is None".to_string()))
        }
        _ => Err(ConversionError::InvalidFieldType(format!(
            "Expected U32, got {:?}",
            ty
        ))),
    }
}

fn u8_from_ty(ty: &Ty) -> Result<u8, ConversionError> {
    match ty {
        Ty::Primitive(Primitive::U8(Some(value))) => Ok(*value),
        Ty::Primitive(Primitive::U8(None)) => {
            Err(ConversionError::InvalidFieldType("U8 is None".to_string()))
        }
        _ => Err(ConversionError::InvalidFieldType(format!(
            "Expected U8, got {:?}",
            ty
        ))),
    }
}

fn bool_from_ty(ty: &Ty) -> Result<bool, ConversionError> {
    match ty {
        Ty::Primitive(Primitive::Bool(Some(value))) => Ok(*value),
        Ty::Primitive(Primitive::Bool(None)) => Err(ConversionError::InvalidFieldType(
            "Bool is None".to_string(),
        )),
        _ => Err(ConversionError::InvalidFieldType(format!(
            "Expected Bool, got {:?}",
            ty
        ))),
    }
}

fn orb_type_from_ty(ty: &Ty) -> Result<OrbType, ConversionError> {
    match ty {
        Ty::Enum(enum_data) => {
            if enum_data.name != "OrbType" {
                return Err(ConversionError::InvalidFieldType(format!(
                    "Expected OrbType enum, got {}",
                    enum_data.name
                )));
            }

            let option = enum_data.option().map_err(|_| {
                ConversionError::InvalidFieldType("Enum option not set".to_string())
            })?;

            match option.name.as_str() {
                "SingleBomb" => Ok(OrbType::SingleBomb),
                "DoubleBomb" => Ok(OrbType::DoubleBomb),
                "TripleBomb" => Ok(OrbType::TripleBomb),
                "FivePoints" => Ok(OrbType::FivePoints),
                "DoubleMultiplier" => Ok(OrbType::DoubleMultiplier),
                "RemainingOrbs" => Ok(OrbType::RemainingOrbs),
                "BombCounter" => Ok(OrbType::BombCounter),
                "Health" => Ok(OrbType::Health),
                "CheddahBomb" => Ok(OrbType::CheddahBomb),
                "SevenPoints" => Ok(OrbType::SevenPoints),
                "MoonRock" => Ok(OrbType::MoonRock),
                "HalfMultiplier" => Ok(OrbType::HalfMultiplier),
                "EightPoints" => Ok(OrbType::EightPoints),
                "NinePoints" => Ok(OrbType::NinePoints),
                "NextPoints2x" => Ok(OrbType::NextPoints2x),
                "Multiplier1_5x" => Ok(OrbType::Multiplier1_5x),
                "BigHealth" => Ok(OrbType::BigHealth),
                "BigMoonRock" => Ok(OrbType::BigMoonRock),
                _ => Err(ConversionError::InvalidFieldType(format!(
                    "Unknown OrbType variant: {}",
                    option.name
                ))),
            }
        }
        _ => Err(ConversionError::InvalidFieldType(format!(
            "Expected Enum, got {:?}",
            ty
        ))),
    }
}

fn game_state_from_ty(ty: &Ty) -> Result<GameState, ConversionError> {
    match ty {
        Ty::Enum(enum_data) => {
            if enum_data.name != "GameState" {
                return Err(ConversionError::InvalidFieldType(format!(
                    "Expected GameState enum, got {}",
                    enum_data.name
                )));
            }

            let option = enum_data.option().map_err(|_| {
                ConversionError::InvalidFieldType("Enum option not set".to_string())
            })?;

            match option.name.as_str() {
                "Active" => Ok(GameState::Active),
                "LevelComplete" => Ok(GameState::LevelComplete),
                "GameWon" => Ok(GameState::GameWon),
                "GameLost" => Ok(GameState::GameLost),
                _ => Err(ConversionError::InvalidFieldType(format!(
                    "Unknown GameState variant: {}",
                    option.name
                ))),
            }
        }
        _ => Err(ConversionError::InvalidFieldType(format!(
            "Expected Enum, got {:?}",
            ty
        ))),
    }
}

fn shop_rarity_from_ty(ty: &Ty) -> Result<ShopRarity, ConversionError> {
    match ty {
        Ty::Enum(enum_data) => {
            if enum_data.name != "ShopRarity" {
                return Err(ConversionError::InvalidFieldType(format!(
                    "Expected ShopRarity enum, got {}",
                    enum_data.name
                )));
            }

            let option = enum_data.option().map_err(|_| {
                ConversionError::InvalidFieldType("Enum option not set".to_string())
            })?;

            match option.name.as_str() {
                "Common" => Ok(ShopRarity::Common),
                "Rare" => Ok(ShopRarity::Rare),
                "Cosmic" => Ok(ShopRarity::Cosmic),
                _ => Err(ConversionError::InvalidFieldType(format!(
                    "Unknown ShopRarity variant: {}",
                    option.name
                ))),
            }
        }
        _ => Err(ConversionError::InvalidFieldType(format!(
            "Expected Enum, got {:?}",
            ty
        ))),
    }
}

impl MoonRocks {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(MoonRocks {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            amount: get_field_value(dojo_struct, "amount", u32_from_ty)?,
        })
    }
}

impl Game {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(Game {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            game_id: get_field_value(dojo_struct, "game_id", u32_from_ty)?,
            health: get_field_value(dojo_struct, "health", u8_from_ty)?,
            points: get_field_value(dojo_struct, "points", u32_from_ty)?,
            multiplier: get_field_value(dojo_struct, "multiplier", u32_from_ty)?,
            cheddah: get_field_value(dojo_struct, "cheddah", u32_from_ty)?,
            current_level: get_field_value(dojo_struct, "current_level", u8_from_ty)?,
            is_active: get_field_value(dojo_struct, "is_active", bool_from_ty)?,
            game_state: get_field_value(dojo_struct, "game_state", game_state_from_ty)?,
            orb_bag_size: get_field_value(dojo_struct, "orb_bag_size", u32_from_ty)?,
            orbs_drawn_count: get_field_value(dojo_struct, "orbs_drawn_count", u32_from_ty)?,
            bombs_drawn_count: get_field_value(dojo_struct, "bombs_drawn_count", u32_from_ty)?,
            temp_multiplier_active: get_field_value(
                dojo_struct,
                "temp_multiplier_active",
                bool_from_ty,
            )?,
            temp_multiplier_value: get_field_value(
                dojo_struct,
                "temp_multiplier_value",
                u32_from_ty,
            )?,
        })
    }
}

impl GameCounter {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(GameCounter {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            next_game_id: get_field_value(dojo_struct, "next_game_id", u32_from_ty)?,
        })
    }
}

impl ActiveGame {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(ActiveGame {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            game_id: get_field_value(dojo_struct, "game_id", u32_from_ty)?,
        })
    }
}

impl OrbBagSlot {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(OrbBagSlot {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            game_id: get_field_value(dojo_struct, "game_id", u32_from_ty)?,
            slot_index: get_field_value(dojo_struct, "slot_index", u32_from_ty)?,
            orb_type: get_field_value(dojo_struct, "orb_type", orb_type_from_ty)?,
            is_active: get_field_value(dojo_struct, "is_active", bool_from_ty)?,
        })
    }
}

impl DrawnOrb {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(DrawnOrb {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            game_id: get_field_value(dojo_struct, "game_id", u32_from_ty)?,
            draw_index: get_field_value(dojo_struct, "draw_index", u32_from_ty)?,
            orb_type: get_field_value(dojo_struct, "orb_type", orb_type_from_ty)?,
        })
    }
}

impl ShopInventory {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(ShopInventory {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            game_id: get_field_value(dojo_struct, "game_id", u32_from_ty)?,
            level: get_field_value(dojo_struct, "level", u8_from_ty)?,
            slot_index: get_field_value(dojo_struct, "slot_index", u8_from_ty)?,
            orb_type: get_field_value(dojo_struct, "orb_type", orb_type_from_ty)?,
            base_price: get_field_value(dojo_struct, "base_price", u32_from_ty)?,
            rarity: get_field_value(dojo_struct, "rarity", shop_rarity_from_ty)?,
        })
    }
}

impl PurchaseHistory {
    pub fn from_dojo_struct(dojo_struct: &Struct) -> Result<Self, ConversionError> {
        Ok(PurchaseHistory {
            player: get_field_value(dojo_struct, "player", felt_from_ty)?,
            game_id: get_field_value(dojo_struct, "game_id", u32_from_ty)?,
            orb_type: get_field_value(dojo_struct, "orb_type", orb_type_from_ty)?,
            purchase_count: get_field_value(dojo_struct, "purchase_count", u32_from_ty)?,
        })
    }
}
