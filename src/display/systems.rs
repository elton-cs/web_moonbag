//! UI update systems that react to torii events

use bevy::prelude::*;
use crate::torii::events::*;
use crate::torii::types::*;
use super::components::*;
use super::styles::*;

/// Setup the initial display UI
pub fn setup_display_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    // Create the main UI root
    commands
        .spawn((
            StyleBuilders::display_root(),
            BackgroundColor(Colors::BACKGROUND),
            BorderColor(Colors::BORDER),
            DojoDisplayRoot,
        ))
        .with_children(|parent| {
            // Game Stats Panel
            create_game_stats_panel(parent, font.clone());
            
            // Moon Rocks Display
            create_moon_rocks_display(parent, font.clone());
            
            // Position Display
            create_position_display(parent, font.clone());
            
            // Game State Display
            create_game_state_display(parent, font.clone());
            
            // Orb Bag Panel
            create_orb_bag_panel(parent, font.clone());
            
            // Shop Panel
            create_shop_panel(parent, font.clone());
            
            // Drawn Orbs Panel
            create_drawn_orbs_panel(parent, font.clone());
        });
}

fn create_game_stats_panel(parent: &mut BuildChildrenCommands, font: Handle<Font>) {
    parent
        .spawn((
            NodeBundle {
                style: StyleBuilders::panel_container(),
                background_color: Colors::PANEL_BACKGROUND.into(),
                border_color: Colors::BORDER.into(),
                ..default()
            },
            GameStatsPanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn(TextBundle::from_section(
                "Game Stats",
                TextStyle {
                    font: font.clone(),
                    font_size: Typography::HEADER_SIZE,
                    color: Colors::TEXT_PRIMARY,
                },
            ));
            
            // Health
            panel.spawn((
                NodeBundle {
                    style: StyleBuilders::row_container(),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(TextBundle::from_section(
                    "Health:",
                    TextStyle {
                        font: font.clone(),
                        font_size: Typography::BODY_SIZE,
                        color: Colors::TEXT_SECONDARY,
                    },
                ));
                row.spawn((
                    TextBundle::from_section(
                        "0",
                        TextStyle {
                            font: font.clone(),
                            font_size: Typography::BODY_SIZE,
                            color: Colors::ACCENT_RED,
                        },
                    ),
                    HealthDisplay,
                ));
            });
            
            // Points
            panel.spawn((
                NodeBundle {
                    style: StyleBuilders::row_container(),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(TextBundle::from_section(
                    "Points:",
                    TextStyle {
                        font: font.clone(),
                        font_size: Typography::BODY_SIZE,
                        color: Colors::TEXT_SECONDARY,
                    },
                ));
                row.spawn((
                    TextBundle::from_section(
                        "0",
                        TextStyle {
                            font: font.clone(),
                            font_size: Typography::BODY_SIZE,
                            color: Colors::ACCENT_BLUE,
                        },
                    ),
                    PointsDisplay,
                ));
            });
            
            // Multiplier
            panel.spawn((
                NodeBundle {
                    style: StyleBuilders::row_container(),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(TextBundle::from_section(
                    "Multiplier:",
                    TextStyle {
                        font: font.clone(),
                        font_size: Typography::BODY_SIZE,
                        color: Colors::TEXT_SECONDARY,
                    },
                ));
                row.spawn((
                    TextBundle::from_section(
                        "1x",
                        TextStyle {
                            font: font.clone(),
                            font_size: Typography::BODY_SIZE,
                            color: Colors::ACCENT_GREEN,
                        },
                    ),
                    MultiplierDisplay,
                ));
            });
            
            // Level
            panel.spawn((
                NodeBundle {
                    style: StyleBuilders::row_container(),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(TextBundle::from_section(
                    "Level:",
                    TextStyle {
                        font: font.clone(),
                        font_size: Typography::BODY_SIZE,
                        color: Colors::TEXT_SECONDARY,
                    },
                ));
                row.spawn((
                    TextBundle::from_section(
                        "1",
                        TextStyle {
                            font: font.clone(),
                            font_size: Typography::BODY_SIZE,
                            color: Colors::ACCENT_YELLOW,
                        },
                    ),
                    LevelDisplay,
                ));
            });
            
            // Cheddah
            panel.spawn((
                NodeBundle {
                    style: StyleBuilders::row_container(),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(TextBundle::from_section(
                    "Cheddah:",
                    TextStyle {
                        font: font.clone(),
                        font_size: Typography::BODY_SIZE,
                        color: Colors::TEXT_SECONDARY,
                    },
                ));
                row.spawn((
                    TextBundle::from_section(
                        "0",
                        TextStyle {
                            font: font.clone(),
                            font_size: Typography::BODY_SIZE,
                            color: Colors::ACCENT_YELLOW,
                        },
                    ),
                    CheddahDisplay,
                ));
            });
        });
}

fn create_moon_rocks_display(parent: &mut BuildChildrenCommands, font: Handle<Font>) {
    parent.spawn((
            Text::new("Moon Rocks: 0"),
            TextFont {
                font: font.clone(),
                font_size: Typography::BODY_SIZE,
                ..default()
            },
            TextColor(Colors::ACCENT_BLUE),
            MoonRocksDisplay,
    ));
}

fn create_position_display(parent: &mut BuildChildrenCommands, font: Handle<Font>) {
    parent.spawn((
        TextBundle::from_section(
            "Position: (0, 0)",
            TextStyle {
                font: font.clone(),
                font_size: Typography::BODY_SIZE,
                color: Colors::TEXT_PRIMARY,
            },
        ),
        PositionDisplay,
    ));
}

fn create_game_state_display(parent: &mut BuildChildrenCommands, font: Handle<Font>) {
    parent.spawn((
        TextBundle::from_section(
            "Game State: Inactive",
            TextStyle {
                font: font.clone(),
                font_size: Typography::BODY_SIZE,
                color: Colors::TEXT_PRIMARY,
            },
        ),
        GameStateDisplay,
    ));
}

fn create_orb_bag_panel(parent: &mut BuildChildrenCommands, font: Handle<Font>) {
    parent
        .spawn((
            NodeBundle {
                style: StyleBuilders::panel_container(),
                background_color: Colors::PANEL_BACKGROUND.into(),
                border_color: Colors::BORDER.into(),
                ..default()
            },
            OrbBagPanel,
        ))
        .with_children(|panel| {
            panel.spawn(TextBundle::from_section(
                "Orb Bag",
                TextStyle {
                    font: font.clone(),
                    font_size: Typography::HEADER_SIZE,
                    color: Colors::TEXT_PRIMARY,
                },
            ));
            
            // Create a grid for orb slots
            panel.spawn(NodeBundle {
                style: StyleBuilders::orb_bag_grid(),
                ..default()
            });
        });
}

fn create_shop_panel(parent: &mut BuildChildrenCommands, font: Handle<Font>) {
    parent
        .spawn((
            NodeBundle {
                style: StyleBuilders::panel_container(),
                background_color: Colors::PANEL_BACKGROUND.into(),
                border_color: Colors::BORDER.into(),
                ..default()
            },
            ShopPanel,
        ))
        .with_children(|panel| {
            panel.spawn(TextBundle::from_section(
                "Shop",
                TextStyle {
                    font: font.clone(),
                    font_size: Typography::HEADER_SIZE,
                    color: Colors::TEXT_PRIMARY,
                },
            ));
        });
}

fn create_drawn_orbs_panel(parent: &mut BuildChildrenCommands, font: Handle<Font>) {
    parent
        .spawn((
            NodeBundle {
                style: StyleBuilders::panel_container(),
                background_color: Colors::PANEL_BACKGROUND.into(),
                border_color: Colors::BORDER.into(),
                ..default()
            },
            DrawnOrbsPanel,
        ))
        .with_children(|panel| {
            panel.spawn(TextBundle::from_section(
                "Drawn Orbs",
                TextStyle {
                    font: font.clone(),
                    font_size: Typography::HEADER_SIZE,
                    color: Colors::TEXT_PRIMARY,
                },
            ));
        });
}

/// Update game stats display when game data changes
pub fn update_game_stats_display(
    mut events: EventReader<GameUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut health_query: Query<&mut Text, (With<HealthDisplay>, Without<PointsDisplay>, Without<MultiplierDisplay>, Without<LevelDisplay>, Without<CheddahDisplay>)>,
    mut points_query: Query<&mut Text, (With<PointsDisplay>, Without<HealthDisplay>, Without<MultiplierDisplay>, Without<LevelDisplay>, Without<CheddahDisplay>)>,
    mut multiplier_query: Query<&mut Text, (With<MultiplierDisplay>, Without<HealthDisplay>, Without<PointsDisplay>, Without<LevelDisplay>, Without<CheddahDisplay>)>,
    mut level_query: Query<&mut Text, (With<LevelDisplay>, Without<HealthDisplay>, Without<PointsDisplay>, Without<MultiplierDisplay>, Without<CheddahDisplay>)>,
    mut cheddah_query: Query<&mut Text, (With<CheddahDisplay>, Without<HealthDisplay>, Without<PointsDisplay>, Without<MultiplierDisplay>, Without<LevelDisplay>)>,
) {
    for event in events.read() {
        display_data.current_game = Some(event.0.clone());
        let game = &event.0;
        
        // Update health display
        if let Ok(mut text) = health_query.single_mut() {
            text.0[0].value = game.health.to_string();
        }
        
        // Update points display
        if let Ok(mut text) = points_query.single_mut() {
            text.0[0].value = game.points.to_string();
        }
        
        // Update multiplier display
        if let Ok(mut text) = multiplier_query.single_mut() {
            text.0[0].value = format!("{}x", game.multiplier);
        }
        
        // Update level display  
        if let Ok(mut text) = level_query.single_mut() {
            text.0[0].value = game.current_level.to_string();
        }
        
        // Update cheddah display
        if let Ok(mut text) = cheddah_query.single_mut() {
            text.0[0].value = game.cheddah.to_string();
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
        
        // Find the moon rocks text and update it
        for mut text in query.iter_mut() {
            text.0[0].value = format!("Moon Rocks: {}", event.0.amount);
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
        
        // Find the position text and update it
        for mut text in query.iter_mut() {
            text.0[0].value = format!("Position: ({}, {})", event.0.x, event.0.y);
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
        
        // Find the game state text and update it
        for mut text in query.iter_mut() {
            text.0[0].value = format!("Game State: {}", state_text);
        }
    }
}

/// Update orb bag display
pub fn update_orb_bag_display(
    mut events: EventReader<OrbBagSlotUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
) {
    for event in events.read() {
        let slot = event.0.clone();
        
        // Update the display data
        if let Some(existing_slot) = display_data.orb_bag_slots.iter_mut().find(|s| s.slot_index == slot.slot_index) {
            *existing_slot = slot.clone();
        } else {
            display_data.orb_bag_slots.push(slot.clone());
        }
        
        // For now, just track the data - complex UI updates can be added later
        // In a full implementation, we would rebuild the entire grid when slots change
    }
}

/// Update shop display
pub fn update_shop_display(
    mut events: EventReader<ShopInventoryUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut commands: Commands,
    shop_query: Query<Entity, With<ShopPanel>>,
    shop_item_query: Query<Entity, With<ShopItemDisplay>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let item = event.0.clone();
        
        // Update the display data
        if let Some(existing_item) = display_data.shop_items.iter_mut().find(|i| i.slot_index == item.slot_index) {
            *existing_item = item.clone();
        } else {
            display_data.shop_items.push(item.clone());
        }
        
        // Update the visual display
        for panel_entity in shop_query.iter() {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            
            // Add the shop item display
            commands.entity(panel_entity).with_children(|panel| {
                panel.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            padding: UiRect::all(Val::Px(Spacing::SMALL)),
                            margin: UiRect::vertical(Val::Px(Spacing::SMALL)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Colors::PANEL_BACKGROUND.into(),
                        border_color: Colors::BORDER.into(),
                        ..default()
                    },
                    ShopItemDisplay {
                        slot_index: item.slot_index,
                    },
                )).with_children(|item_ui| {
                    // Item name/type
                    item_ui.spawn(TextBundle::from_section(
                        format_orb_type(&item.orb_type),
                        TextStyle {
                            font: font.clone(),
                            font_size: Typography::BODY_SIZE,
                            color: Colors::TEXT_PRIMARY,
                        },
                    ));
                    
                    // Price
                    item_ui.spawn(TextBundle::from_section(
                        format!("{} coins", item.base_price),
                        TextStyle {
                            font: font.clone(),
                            font_size: Typography::BODY_SIZE,
                            color: Colors::ACCENT_YELLOW,
                        },
                    ));
                });
            });
        }
    }
}

/// Update drawn orbs display
pub fn update_drawn_orbs_display(
    mut events: EventReader<DrawnOrbAddedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut commands: Commands,
    drawn_orbs_query: Query<Entity, With<DrawnOrbsPanel>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let drawn_orb = event.0.clone();
        display_data.drawn_orbs.push(drawn_orb.clone());
        
        // Keep only the last 10 drawn orbs for display
        if display_data.drawn_orbs.len() > 10 {
            display_data.drawn_orbs.remove(0);
        }
        
        // Find the drawn orbs panel and add the new orb
        for panel_entity in drawn_orbs_query.iter() {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            commands.entity(panel_entity).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    format!("Orb #{}: {}", drawn_orb.draw_index, format_orb_type(&drawn_orb.orb_type)),
                    TextStyle {
                        font: font.clone(),
                        font_size: Typography::SMALL_SIZE,
                        color: Colors::TEXT_SECONDARY,
                    },
                ));
            });
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