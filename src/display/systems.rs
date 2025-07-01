//! Simple UI update systems for displaying all Torii events using modern Bevy 0.16 syntax

use bevy::prelude::*;
use crate::torii::events::*;
use crate::torii::types::*;
use super::components::*;
use super::styles::*;

/// Setup the simple event data view UI
pub fn setup_display_ui(mut commands: Commands) {
    // Main container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
            ..default()
        },
        BackgroundColor(Colors::BACKGROUND),
        DojoDisplayRoot,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("🌙 Web Moonbag - Blockchain Game Data"),
            TextFont {
                font_size: Typography::TITLE_SIZE,
                ..default()
            },
            TextColor(Colors::ACCENT_BLUE),
            Node {
                margin: UiRect::bottom(Val::Px(Spacing::LARGE)),
                ..default()
            },
        ));

        // Game Stats Section
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(Spacing::LARGE)),
                padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Colors::PANEL_BACKGROUND),
            BorderColor(Colors::ACCENT_GREEN),
        )).with_children(|section| {
            // Game Stats Header
            section.spawn((
                Text::new("🎮 Game Statistics"),
                TextFont {
                    font_size: Typography::HEADER_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_GREEN),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::MEDIUM)),
                    ..default()
                },
            ));

            // Health Display
            section.spawn((
                Text::new("Health: 0"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_RED),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                HealthDisplay,
            ));

            // Points Display
            section.spawn((
                Text::new("Points: 0"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_YELLOW),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                PointsDisplay,
            ));

            // Level Display
            section.spawn((
                Text::new("Level: 1"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_BLUE),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                LevelDisplay,
            ));

            // Multiplier Display
            section.spawn((
                Text::new("Multiplier: 1x"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                MultiplierDisplay,
            ));

            // Cheddah Display
            section.spawn((
                Text::new("Cheddah: 0"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_YELLOW),
                CheddahDisplay,
            ));
        });

        // Player Data Section
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(Spacing::LARGE)),
                padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Colors::PANEL_BACKGROUND),
            BorderColor(Colors::ACCENT_BLUE),
        )).with_children(|section| {
            // Player Data Header
            section.spawn((
                Text::new("👤 Player Data"),
                TextFont {
                    font_size: Typography::HEADER_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_BLUE),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::MEDIUM)),
                    ..default()
                },
            ));

            // Position Display
            section.spawn((
                Text::new("Position: (0, 0)"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_YELLOW),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                PositionDisplay,
            ));

            // Moon Rocks Display
            section.spawn((
                Text::new("Moon Rocks: 0"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                MoonRocksDisplay,
            ));

            // Game Counter Display
            section.spawn((
                Text::new("Next Game ID: 0"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::TEXT_SECONDARY),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                GameCounterDisplay,
            ));

            // Active Game Display
            section.spawn((
                Text::new("Active Game: None"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::TEXT_SECONDARY),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                ActiveGameDisplay,
            ));

            // Game State Display
            section.spawn((
                Text::new("Game State: Inactive"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::TEXT_SECONDARY),
                GameStateDisplay,
            ));
        });

        // Game Objects Section
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                margin: UiRect::bottom(Val::Px(Spacing::LARGE)),
                padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Colors::PANEL_BACKGROUND),
            BorderColor(Colors::ACCENT_GREEN),
        )).with_children(|section| {
            // Game Objects Header
            section.spawn((
                Text::new("🎯 Game Objects"),
                TextFont {
                    font_size: Typography::HEADER_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_GREEN),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::MEDIUM)),
                    ..default()
                },
            ));

            // Orb Bag Panel
            section.spawn((
                Text::new("Orb Bag Slots: (empty)"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::ACCENT_GREEN),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                OrbBagPanel,
            ));

            // Shop Panel
            section.spawn((
                Text::new("Shop Items: (empty)"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.5, 0.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                ShopPanel,
            ));

            // Drawn Orbs Panel
            section.spawn((
                Text::new("Recent Drawn Orbs: (empty)"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::SMALL)),
                    ..default()
                },
                DrawnOrbsPanel,
            ));

            // Purchase History Panel
            section.spawn((
                Text::new("Purchase History: (empty)"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 1.0)),
                PurchaseHistoryPanel,
            ));
        });

        // Controls Information
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Colors::PANEL_BACKGROUND),
            BorderColor(Colors::BORDER),
        )).with_children(|section| {
            section.spawn((
                Text::new("🎮 Controls"),
                TextFont {
                    font_size: Typography::HEADER_SIZE,
                    ..default()
                },
                TextColor(Colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(Spacing::MEDIUM)),
                    ..default()
                },
            ));

            section.spawn((
                Text::new("C - Connect to blockchain\nSpace - Spawn player\nS - Subscribe to entities\nArrow Keys - Move player"),
                TextFont {
                    font_size: Typography::BODY_SIZE,
                    ..default()
                },
                TextColor(Colors::TEXT_SECONDARY),
            ));
        });
    });
}

/// Update game stats display when game data changes
pub fn update_game_stats_display(
    mut events: EventReader<GameUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut health_query: Query<&mut Text, (With<HealthDisplay>, Without<PointsDisplay>, Without<LevelDisplay>, Without<MultiplierDisplay>, Without<CheddahDisplay>)>,
    mut points_query: Query<&mut Text, (With<PointsDisplay>, Without<HealthDisplay>, Without<LevelDisplay>, Without<MultiplierDisplay>, Without<CheddahDisplay>)>,
    mut level_query: Query<&mut Text, (With<LevelDisplay>, Without<HealthDisplay>, Without<PointsDisplay>, Without<MultiplierDisplay>, Without<CheddahDisplay>)>,
    mut multiplier_query: Query<&mut Text, (With<MultiplierDisplay>, Without<HealthDisplay>, Without<PointsDisplay>, Without<LevelDisplay>, Without<CheddahDisplay>)>,
    mut cheddah_query: Query<&mut Text, (With<CheddahDisplay>, Without<HealthDisplay>, Without<PointsDisplay>, Without<LevelDisplay>, Without<MultiplierDisplay>)>,
) {
    for event in events.read() {
        display_data.current_game = Some(event.0.clone());
        let game = &event.0;
        
        // Update individual displays
        for mut text in health_query.iter_mut() {
            *text = Text::new(format!("Health: {}", game.health));
        }
        
        for mut text in points_query.iter_mut() {
            *text = Text::new(format!("Points: {}", game.points));
        }
        
        for mut text in level_query.iter_mut() {
            *text = Text::new(format!("Level: {}", game.current_level));
        }
        
        for mut text in multiplier_query.iter_mut() {
            *text = Text::new(format!("Multiplier: {}x", game.multiplier));
        }
        
        for mut text in cheddah_query.iter_mut() {
            *text = Text::new(format!("Cheddah: {}", game.cheddah));
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

/// Update game counter display
pub fn update_game_counter_display(
    mut events: EventReader<GameCounterUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<GameCounterDisplay>>,
) {
    for event in events.read() {
        display_data.game_counter = Some(event.0.clone());
        
        for mut text in query.iter_mut() {
            *text = Text::new(format!("Next Game ID: {}", event.0.next_game_id));
        }
    }
}

/// Update active game display
pub fn update_active_game_display(
    mut events: EventReader<ActiveGameUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<ActiveGameDisplay>>,
) {
    for event in events.read() {
        display_data.active_game = Some(event.0.clone());
        
        for mut text in query.iter_mut() {
            *text = Text::new(format!("Active Game: {}", event.0.game_id));
        }
    }
}

/// Update orb bag display by showing a summary
pub fn update_orb_bag_display(
    mut events: EventReader<OrbBagSlotUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<OrbBagPanel>>,
) {
    for event in events.read() {
        let slot = event.0.clone();
        
        // Update the display data
        if let Some(existing_slot) = display_data.orb_bag_slots.iter_mut().find(|s| s.slot_index == slot.slot_index) {
            *existing_slot = slot.clone();
        } else {
            display_data.orb_bag_slots.push(slot.clone());
        }
        
        // Update summary display
        let active_slots: Vec<_> = display_data.orb_bag_slots.iter()
            .filter(|s| s.is_active)
            .collect();
            
        for mut text in query.iter_mut() {
            if active_slots.is_empty() {
                *text = Text::new("Orb Bag Slots: (empty)".to_string());
            } else {
                let slot_summaries: Vec<String> = active_slots.iter()
                    .map(|s| format!("{}: {}", s.slot_index, format_orb_type(&s.orb_type)))
                    .collect();
                *text = Text::new(format!("Orb Bag Slots: {}", slot_summaries.join(", ")));
            }
        }
    }
}

/// Update shop display by showing a summary
pub fn update_shop_display(
    mut events: EventReader<ShopInventoryUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<ShopPanel>>,
) {
    for event in events.read() {
        let item = event.0.clone();
        
        // Update the display data
        if let Some(existing_item) = display_data.shop_items.iter_mut().find(|i| i.slot_index == item.slot_index) {
            *existing_item = item.clone();
        } else {
            display_data.shop_items.push(item.clone());
        }
        
        // Update summary display
        for mut text in query.iter_mut() {
            if display_data.shop_items.is_empty() {
                *text = Text::new("Shop Items: (empty)".to_string());
            } else {
                let item_summaries: Vec<String> = display_data.shop_items.iter()
                    .take(3) // Show only first 3 items
                    .map(|i| format!("{} ({})", format_orb_type(&i.orb_type), i.base_price))
                    .collect();
                let summary = if display_data.shop_items.len() > 3 {
                    format!("Shop Items: {} (+{} more)", item_summaries.join(", "), display_data.shop_items.len() - 3)
                } else {
                    format!("Shop Items: {}", item_summaries.join(", "))
                };
                *text = Text::new(summary);
            }
        }
    }
}

/// Update drawn orbs display by showing a summary
pub fn update_drawn_orbs_display(
    mut events: EventReader<DrawnOrbUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<DrawnOrbsPanel>>,
) {
    for event in events.read() {
        let drawn_orb = event.0.clone();
        display_data.drawn_orbs.push(drawn_orb.clone());
        
        // Keep only the last 10 drawn orbs for display
        if display_data.drawn_orbs.len() > 10 {
            display_data.drawn_orbs.remove(0);
        }
        
        // Update summary display
        for mut text in query.iter_mut() {
            if display_data.drawn_orbs.is_empty() {
                *text = Text::new("Recent Drawn Orbs: (empty)".to_string());
            } else {
                let recent_orbs: Vec<String> = display_data.drawn_orbs.iter()
                    .rev()
                    .take(3) // Show last 3 orbs
                    .map(|o| format_orb_type(&o.orb_type).to_string())
                    .collect();
                let summary = if display_data.drawn_orbs.len() > 3 {
                    format!("Recent Drawn Orbs: {} (+{} more)", recent_orbs.join(", "), display_data.drawn_orbs.len() - 3)
                } else {
                    format!("Recent Drawn Orbs: {}", recent_orbs.join(", "))
                };
                *text = Text::new(summary);
            }
        }
    }
}

/// Update purchase history display by showing a summary
pub fn update_purchase_history_display(
    mut events: EventReader<PurchaseHistoryUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<PurchaseHistoryPanel>>,
) {
    for event in events.read() {
        let history = event.0.clone();
        display_data.purchase_history.push(history.clone());
        
        // Update summary display
        for mut text in query.iter_mut() {
            if display_data.purchase_history.is_empty() {
                *text = Text::new("Purchase History: (empty)".to_string());
            } else {
                let recent_purchases: Vec<String> = display_data.purchase_history.iter()
                    .rev()
                    .take(3) // Show last 3 purchases
                    .map(|p| format!("{} x{}", format_orb_type(&p.orb_type), p.purchase_count))
                    .collect();
                let summary = if display_data.purchase_history.len() > 3 {
                    format!("Purchase History: {} (+{} more)", recent_purchases.join(", "), display_data.purchase_history.len() - 3)
                } else {
                    format!("Purchase History: {}", recent_purchases.join(", "))
                };
                *text = Text::new(summary);
            }
        }
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