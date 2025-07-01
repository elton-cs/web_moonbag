//! UI update systems that react to torii events using modern Bevy 0.16 syntax

use bevy::prelude::*;
use crate::torii::events::*;
use crate::torii::types::*;
use super::components::*;

/// Setup the initial display UI
pub fn setup_display_ui(mut commands: Commands) {
    // Game Stats Display
    commands.spawn((
        Text::new("Health: 0 | Points: 0 | Multiplier: 1x | Level: 1 | Cheddah: 0"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        HealthDisplay,
    ));
    
    // Moon Rocks Display
    commands.spawn((
        Text::new("Moon Rocks: 0"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 1.0)), // Cyan
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        },
        MoonRocksDisplay,
    ));
    
    // Position Display
    commands.spawn((
        Text::new("Position: (0, 0)"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)), // Yellow
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            ..default()
        },
        PositionDisplay,
    ));
    
    // Game State Display
    commands.spawn((
        Text::new("Game State: Inactive"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)), // Light Gray
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(10.0),
            ..default()
        },
        GameStateDisplay,
    ));
    
    // Orb Bag Panel Header
    commands.spawn((
        Text::new("Orb Bag Slots:"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.0)), // Green
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(140.0),
            left: Val::Px(10.0),
            ..default()
        },
        OrbBagPanel,
    ));
    
    // Shop Panel Header
    commands.spawn((
        Text::new("Shop Items:"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.5, 0.0)), // Orange
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(10.0),
            ..default()
        },
        ShopPanel,
    ));
    
    // Drawn Orbs Panel Header
    commands.spawn((
        Text::new("Recent Drawn Orbs:"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 1.0)), // Magenta
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(500.0),
            left: Val::Px(10.0),
            ..default()
        },
        DrawnOrbsPanel,
    ));
}

/// Update game stats display when game data changes
pub fn update_game_stats_display(
    mut events: EventReader<GameUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<HealthDisplay>>,
) {
    for event in events.read() {
        display_data.current_game = Some(event.0.clone());
        let game = &event.0;
        
        for mut text in query.iter_mut() {
            *text = Text::new(format!(
                "Health: {} | Points: {} | Multiplier: {}x | Level: {} | Cheddah: {}",
                game.health, game.points, game.multiplier, game.current_level, game.cheddah
            ));
        }
    }
}

/// Update moon rocks display
pub fn update_moon_rocks_display(
    mut events: EventReader<MoonRocksUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<MoonRocksDisplay>>,
) {
    for event in events.read() {
        display_data.moon_rocks = Some(event.0.clone());
        
        for mut text in query.iter_mut() {
            *text = Text::new(format!("Moon Rocks: {}", event.0.amount));
        }
    }
}

/// Update position display
pub fn update_position_display(
    mut events: EventReader<PositionUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<PositionDisplay>>,
) {
    for event in events.read() {
        display_data.position = Some(event.0);
        
        for mut text in query.iter_mut() {
            *text = Text::new(format!("Position: ({}, {})", event.0.x, event.0.y));
        }
    }
}

/// Update game state display
pub fn update_game_state_display(
    mut events: EventReader<GameUpdatedEvent>,
    mut query: Query<&mut Text, With<GameStateDisplay>>,
) {
    for event in events.read() {
        let state_text = match event.0.game_state {
            GameState::Active => "Active",
            GameState::LevelComplete => "Level Complete",
            GameState::GameWon => "Game Won",
            GameState::GameLost => "Game Lost",
        };
        
        for mut text in query.iter_mut() {
            *text = Text::new(format!("Game State: {}", state_text));
        }
    }
}

/// Update orb bag display by adding new slot information
pub fn update_orb_bag_display(
    mut events: EventReader<OrbBagSlotUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut commands: Commands,
    _orb_bag_query: Query<Entity, With<OrbBagPanel>>,
) {
    for event in events.read() {
        let slot = event.0.clone();
        
        // Update the display data
        if let Some(existing_slot) = display_data.orb_bag_slots.iter_mut().find(|s| s.slot_index == slot.slot_index) {
            *existing_slot = slot.clone();
        } else {
            display_data.orb_bag_slots.push(slot.clone());
        }
        
        // Add text display for this slot (simplified approach)
        let y_offset = 170.0 + (slot.slot_index as f32 * 25.0);
        commands.spawn((
            Text::new(format!(
                "Slot {}: {} (Active: {})",
                slot.slot_index,
                format_orb_type(&slot.orb_type),
                slot.is_active
            )),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 1.0, 0.7)), // Light Green
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(y_offset),
                left: Val::Px(20.0),
                ..default()
            },
            OrbBagSlotDisplay {
                slot_index: slot.slot_index,
            },
        ));
    }
}

/// Update shop display by adding new shop items
pub fn update_shop_display(
    mut events: EventReader<ShopInventoryUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut commands: Commands,
) {
    for event in events.read() {
        let item = event.0.clone();
        
        // Update the display data
        if let Some(existing_item) = display_data.shop_items.iter_mut().find(|i| i.slot_index == item.slot_index) {
            *existing_item = item.clone();
        } else {
            display_data.shop_items.push(item.clone());
        }
        
        // Add text display for this shop item
        let y_offset = 330.0 + (item.slot_index as f32 * 25.0);
        commands.spawn((
            Text::new(format!(
                "Shop {}: {} - {} coins (Rarity: {:?})",
                item.slot_index,
                format_orb_type(&item.orb_type),
                item.base_price,
                item.rarity
            )),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.8, 0.4)), // Light Orange
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(y_offset),
                left: Val::Px(20.0),
                ..default()
            },
            ShopItemDisplay {
                slot_index: item.slot_index,
            },
        ));
    }
}

/// Update drawn orbs display by adding new orb history
pub fn update_drawn_orbs_display(
    mut events: EventReader<DrawnOrbAddedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut commands: Commands,
) {
    for event in events.read() {
        let drawn_orb = event.0.clone();
        display_data.drawn_orbs.push(drawn_orb.clone());
        
        // Keep only the last 10 drawn orbs for display
        if display_data.drawn_orbs.len() > 10 {
            display_data.drawn_orbs.remove(0);
        }
        
        // Add text display for this drawn orb
        let orb_count = display_data.drawn_orbs.len();
        let y_offset = 530.0 + ((orb_count - 1) as f32 * 20.0);
        commands.spawn((
            Text::new(format!(
                "Draw #{}: {} (Game: {})",
                drawn_orb.draw_index,
                format_orb_type(&drawn_orb.orb_type),
                drawn_orb.game_id
            )),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.7, 1.0)), // Light Magenta
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(y_offset),
                left: Val::Px(20.0),
                ..default()
            },
        ));
    }
}

/// Update game counter display
pub fn update_game_counter_display(
    mut events: EventReader<GameCounterUpdatedEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        let counter = &event.0;
        
        commands.spawn((
            Text::new(format!("Next Game ID: {}", counter.next_game_id)),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 1.0)), // Light Blue
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(130.0),
                left: Val::Px(10.0),
                ..default()
            },
        ));
    }
}

/// Update active game display
pub fn update_active_game_display(
    mut events: EventReader<ActiveGameUpdatedEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        let active_game = &event.0;
        
        commands.spawn((
            Text::new(format!("Active Game: {}", active_game.game_id)),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.8)), // Light Yellow
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(115.0),
                left: Val::Px(10.0),
                ..default()
            },
        ));
    }
}

/// Helper function to format orb type names
pub fn format_orb_type(orb_type: &OrbType) -> &'static str {
    match orb_type {
        OrbType::SingleBomb => "Single Bomb",
        OrbType::DoubleBomb => "Double Bomb",
        OrbType::TripleBomb => "Triple Bomb",
        OrbType::FivePoints => "5 Points",
        OrbType::DoubleMultiplier => "2x Multiplier",
        OrbType::RemainingOrbs => "Remaining Orbs",
        OrbType::BombCounter => "Bomb Counter",
        OrbType::Health => "Health",
        OrbType::CheddahBomb => "Cheddah Bomb",
        OrbType::SevenPoints => "7 Points",
        OrbType::MoonRock => "Moon Rock",
        OrbType::HalfMultiplier => "0.5x Multiplier",
        OrbType::EightPoints => "8 Points",
        OrbType::NinePoints => "9 Points",
        OrbType::NextPoints2x => "Next Points 2x",
        OrbType::Multiplier1_5x => "1.5x Multiplier",
        OrbType::BigHealth => "Big Health",
        OrbType::BigMoonRock => "Big Moon Rock",
    }
}