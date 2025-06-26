use bevy::prelude::*;
use bevy::ui::Val::*;
use starknet::core::types::Felt;
use std::collections::HashSet;

use super::game_state::GameState;

pub struct GameStateDisplayPlugin;

impl Plugin for GameStateDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerNavigation::default())
            .add_systems(Startup, (spawn_game_state_display, spawn_start_game_button))
            .add_systems(
                Update,
                (
                    collect_available_players,
                    update_game_state_display,
                    handle_navigation_buttons,
                    handle_start_game_button,
                    handle_escape_button,
                ),
            );
    }
}

#[derive(Component)]
struct GameStateDisplay;

#[derive(Component)]
struct GameStateDisplayRoot;

#[derive(Component)]
struct MoonRocksText;

#[derive(Component)]
struct ActiveGamesText;

#[derive(Component)]
struct GameCountersText;

#[derive(Component)]
struct OrbBagSlotsText;

#[derive(Component)]
struct ShopInventoryText;

#[derive(Component)]
struct PurchaseHistoryText;

#[derive(Component)]
struct PrevPlayerButton;

#[derive(Component)]
struct NextPlayerButton;

#[derive(Component)]
struct PlayerInfoText;

#[derive(Component)]
struct StartGameButton;

#[derive(Component)]
struct EscapeButton;

#[derive(Resource, Default)]
struct PlayerNavigation {
    available_players: Vec<Felt>,
    current_index: usize,
}

impl PlayerNavigation {
    fn get_current_player(&self) -> Option<Felt> {
        self.available_players.get(self.current_index).copied()
    }

    fn next_player(&mut self) {
        if !self.available_players.is_empty() {
            self.current_index = (self.current_index + 1) % self.available_players.len();
        }
    }

    fn prev_player(&mut self) {
        if !self.available_players.is_empty() {
            self.current_index = if self.current_index == 0 {
                self.available_players.len() - 1
            } else {
                self.current_index - 1
            };
        }
    }

    fn update_players(&mut self, players: Vec<Felt>) {
        let current_player = self.get_current_player();
        self.available_players = players;

        if let Some(current) = current_player {
            if let Some(index) = self.available_players.iter().position(|&p| p == current) {
                self.current_index = index;
            } else {
                self.current_index = 0;
            }
        } else {
            self.current_index = 0;
        }
    }
}

fn spawn_game_state_display(mut commands: Commands) {
    // Root container
    commands
        .spawn((
            GameStateDisplayRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Percent(100.0),
                height: Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            Visibility::Hidden,
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            // Main display container
            parent
                .spawn((
                    GameStateDisplay,
                    Node {
                        width: Percent(90.0),
                        height: Percent(90.0),
                        max_width: Px(1200.0),
                        max_height: Px(800.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Px(15.0)),
                        row_gap: Px(10.0),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
                    BorderRadius::all(Px(10.0)),
                ))
                .with_children(|parent| {
                    // Navigation Header
                    parent
                        .spawn((Node {
                            width: Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::bottom(Px(10.0)),
                            ..default()
                        },))
                        .with_children(|parent| {
                            // Previous button
                            parent
                                .spawn((
                                    PrevPlayerButton,
                                    Button,
                                    Node {
                                        width: Px(80.0),
                                        height: Px(30.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                    BorderRadius::all(Px(5.0)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("< Prev"),
                                        TextFont::from_font_size(12.0),
                                        TextColor(Color::WHITE),
                                    ));
                                });

                            // Player info
                            parent.spawn((
                                PlayerInfoText,
                                Text::new("No Players"),
                                TextFont::from_font_size(16.0),
                                TextColor(Color::WHITE),
                                Node {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                            ));

                            // Next button
                            parent
                                .spawn((
                                    NextPlayerButton,
                                    Button,
                                    Node {
                                        width: Px(80.0),
                                        height: Px(30.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                                    BorderRadius::all(Px(5.0)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("Next >"),
                                        TextFont::from_font_size(12.0),
                                        TextColor(Color::WHITE),
                                    ));
                                });
                        });

                    // Title and Escape Button Row
                    parent
                        .spawn((Node {
                            width: Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::bottom(Px(15.0)),
                            ..default()
                        },))
                        .with_children(|parent| {
                            // Empty space for balance
                            parent.spawn((Node {
                                width: Px(80.0),
                                ..default()
                            },));

                            // Title
                            parent.spawn((
                                Text::new("Player Game Data"),
                                TextFont::from_font_size(20.0),
                                TextColor(Color::WHITE),
                                Node {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                            ));

                            // Escape Button
                            parent
                                .spawn((
                                    EscapeButton,
                                    Button,
                                    Node {
                                        width: Px(80.0),
                                        height: Px(30.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 0.9)), // Dark red background
                                    BorderColor(Color::srgb(0.8, 0.3, 0.3)),           // Red border
                                    BorderRadius::all(Px(5.0)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("ESC"),
                                        TextFont::from_font_size(12.0),
                                        TextColor(Color::srgb(1.0, 0.8, 0.8)), // Light red text
                                    ));
                                });
                        });

                    // MoonRocks Section
                    parent.spawn((
                        Text::new("MoonRocks"),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    parent.spawn((
                        MoonRocksText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(12.0),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Px(15.0)),
                            ..default()
                        },
                    ));

                    // Last Game Section
                    parent.spawn((
                        Text::new("Last Game"),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    parent.spawn((
                        ActiveGamesText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(12.0),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Px(15.0)),
                            ..default()
                        },
                    ));

                    // Game Counters Section
                    parent.spawn((
                        Text::new("Game Counters"),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    parent.spawn((
                        GameCountersText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(12.0),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Px(15.0)),
                            ..default()
                        },
                    ));

                    // Orb Bag Slots Section
                    parent.spawn((
                        Text::new("Last Game - Orb Bag Slots"),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    parent.spawn((
                        OrbBagSlotsText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(12.0),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Px(15.0)),
                            ..default()
                        },
                    ));

                    // Shop Inventory Section
                    parent.spawn((
                        Text::new("Last Game - Shop Inventory"),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    parent.spawn((
                        ShopInventoryText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(12.0),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Px(15.0)),
                            ..default()
                        },
                    ));

                    // Purchase History Section
                    parent.spawn((
                        Text::new("Last Game - Purchase History"),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                    parent.spawn((
                        PurchaseHistoryText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(12.0),
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

fn spawn_start_game_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            StartGameButton,
            Button,
            Node {
                position_type: PositionType::Absolute,
                top: Percent(50.0),
                left: Percent(50.0),
                width: Px(200.0),
                height: Px(60.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    left: Px(-100.0), // Half of width to center
                    top: Px(-30.0),   // Half of height to center
                    ..default()
                },
                border: UiRect::all(Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.1, 0.25, 0.9)), // Deep space blue with transparency
            BorderColor(Color::srgb(0.3, 0.6, 1.0)),             // Bright cyan/blue border
            BorderRadius::all(Px(12.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LAUNCH GAME DATA"),
                TextFont {
                    font: asset_server.load("fonts/font1.otf"),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.9, 1.0)), // Slightly bluish white
            ));
        });
}

fn collect_available_players(
    game_state: Option<Res<GameState>>,
    mut player_nav: ResMut<PlayerNavigation>,
    mut player_info_query: Query<&mut Text, With<PlayerInfoText>>,
) {
    let Some(game_state) = game_state else { return };

    let mut players: HashSet<Felt> = HashSet::new();

    // Collect players from all data sources
    for (player, _) in game_state.moon_rocks.iter() {
        players.insert(*player);
    }
    for ((player, _), _) in game_state.games.iter() {
        players.insert(*player);
    }
    for (player, _) in game_state.game_counters.iter() {
        players.insert(*player);
    }
    for (player, _) in game_state.active_games.iter() {
        players.insert(*player);
    }

    let mut players_vec: Vec<Felt> = players.into_iter().collect();
    players_vec.sort_by_key(|p| format!("{:#x}", p));

    if players_vec != player_nav.available_players {
        player_nav.update_players(players_vec);
    }

    // Update player info display
    if let Ok(mut text) = player_info_query.single_mut() {
        if let Some(current_player) = player_nav.get_current_player() {
            **text = format!(
                "Player {} ({}/{})",
                format_felt(&current_player),
                player_nav.current_index + 1,
                player_nav.available_players.len()
            );
        } else {
            **text = "No Players".to_string();
        }
    }
}

fn handle_navigation_buttons(
    mut prev_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PrevPlayerButton>),
    >,
    mut next_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<NextPlayerButton>,
            Without<PrevPlayerButton>,
        ),
    >,
    mut player_nav: ResMut<PlayerNavigation>,
) {
    // Handle prev button
    for (interaction, mut color) in &mut prev_button_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                player_nav.prev_player();
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }

    // Handle next button
    for (interaction, mut color) in &mut next_button_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                player_nav.next_player();
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

fn get_last_game_id(game_state: &GameState, player: &Felt) -> Option<u32> {
    // Find the last game by looking at the game counter
    if let Some(counter) = game_state.game_counters.get(player) {
        if counter.next_game_id > 0 {
            // The last game would be next_game_id - 1
            return Some(counter.next_game_id - 1);
        }
    }

    // Fallback: find the highest game_id for this player in the games map
    game_state
        .games
        .iter()
        .filter_map(
            |((p, game_id), _)| {
                if p == player { Some(*game_id) } else { None }
            },
        )
        .max()
}

fn update_game_state_display(
    game_state: Option<Res<GameState>>,
    player_nav: Res<PlayerNavigation>,
    mut moon_rocks_query: Query<&mut Text, With<MoonRocksText>>,
    mut active_games_query: Query<&mut Text, (With<ActiveGamesText>, Without<MoonRocksText>)>,
    mut game_counters_query: Query<
        &mut Text,
        (
            With<GameCountersText>,
            Without<MoonRocksText>,
            Without<ActiveGamesText>,
        ),
    >,
    mut orb_bag_slots_query: Query<
        &mut Text,
        (
            With<OrbBagSlotsText>,
            Without<MoonRocksText>,
            Without<ActiveGamesText>,
            Without<GameCountersText>,
        ),
    >,
    mut shop_inventory_query: Query<
        &mut Text,
        (
            With<ShopInventoryText>,
            Without<MoonRocksText>,
            Without<ActiveGamesText>,
            Without<GameCountersText>,
            Without<OrbBagSlotsText>,
        ),
    >,
    mut purchase_history_query: Query<
        &mut Text,
        (
            With<PurchaseHistoryText>,
            Without<MoonRocksText>,
            Without<ActiveGamesText>,
            Without<GameCountersText>,
            Without<OrbBagSlotsText>,
            Without<ShopInventoryText>,
        ),
    >,
) {
    let Some(game_state) = game_state else { return };
    let Some(current_player) = player_nav.get_current_player() else {
        // Clear all displays if no player selected
        if let Ok(mut text) = moon_rocks_query.single_mut() {
            **text = "No player selected".to_string();
        }
        if let Ok(mut text) = active_games_query.single_mut() {
            **text = "No player selected".to_string();
        }
        if let Ok(mut text) = game_counters_query.single_mut() {
            **text = "No player selected".to_string();
        }
        if let Ok(mut text) = orb_bag_slots_query.single_mut() {
            **text = "No player selected".to_string();
        }
        if let Ok(mut text) = shop_inventory_query.single_mut() {
            **text = "No player selected".to_string();
        }
        if let Ok(mut text) = purchase_history_query.single_mut() {
            **text = "No player selected".to_string();
        }
        return;
    };

    // Update MoonRocks
    if let Ok(mut text) = moon_rocks_query.single_mut() {
        if let Some(moon_rock) = game_state.moon_rocks.get(&current_player) {
            **text = format!("Amount: {}", moon_rock.amount);
        } else {
            **text = "No MoonRocks data for this player".to_string();
        }
    }

    // Update Last Game (formerly Active Games)
    if let Ok(mut text) = active_games_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            if let Some(game) = game_state.games.get(&(current_player, last_game_id)) {
                **text = format!(
                    "Game #{} | Health: {} | Points: {} | Level: {} | State: {:?}",
                    last_game_id, game.health, game.points, game.current_level, game.game_state
                );
            } else {
                **text = format!("Last game #{} data not found", last_game_id);
            }
        } else {
            **text = "No games found for this player".to_string();
        }
    }

    // Update Game Counters
    if let Ok(mut text) = game_counters_query.single_mut() {
        if let Some(counter) = game_state.game_counters.get(&current_player) {
            **text = format!("Next Game ID: {}", counter.next_game_id);
        } else {
            **text = "No game counter data for this player".to_string();
        }
    }

    // Update Orb Bag Slots (Last Game Only)
    if let Ok(mut text) = orb_bag_slots_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            let mut slots: Vec<&super::types::OrbBagSlot> = game_state
                .orb_bag_slots
                .iter()
                .filter_map(|((player, game_id, _), slot)| {
                    if *player == current_player && *game_id == last_game_id {
                        Some(slot)
                    } else {
                        None
                    }
                })
                .collect();

            // Sort by slot_index
            slots.sort_by_key(|slot| slot.slot_index);

            let mut content = String::new();
            for slot in slots {
                let status = if slot.is_active { "Active" } else { "Inactive" };
                content.push_str(&format!(
                    "Slot {}: {:?} ({})\n",
                    slot.slot_index, slot.orb_type, status
                ));
            }
            if content.is_empty() {
                content = format!("No orb bag slots for last game #{}", last_game_id);
            }
            **text = content;
        } else {
            **text = "No games found for this player".to_string();
        }
    }

    // Update Shop Inventory (Last Game Only)
    if let Ok(mut text) = shop_inventory_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            let mut items_by_level: std::collections::HashMap<
                u8,
                Vec<&super::types::ShopInventory>,
            > = std::collections::HashMap::new();
            for ((player, game_id, level, _), shop) in game_state.shop_inventory.iter() {
                if *player == current_player && *game_id == last_game_id {
                    items_by_level.entry(*level).or_default().push(shop);
                }
            }

            // Convert to sorted vector by level
            let mut sorted_levels: Vec<(u8, Vec<&super::types::ShopInventory>)> =
                items_by_level.into_iter().collect();
            sorted_levels.sort_by_key(|(level, _)| *level);

            let mut content = String::new();
            for (level, mut items) in sorted_levels {
                // Sort items by slot_index within each level
                items.sort_by_key(|item| item.slot_index);
                content.push_str(&format!("Level {} ({} items):\n", level, items.len()));
                for item in items {
                    content.push_str(&format!(
                        "  Slot {}: {:?} - {}🧀 ({:?})\n",
                        item.slot_index, item.orb_type, item.base_price, item.rarity
                    ));
                }
            }
            if content.is_empty() {
                content = format!("No shop inventory for last game #{}", last_game_id);
            }
            **text = content;
        } else {
            **text = "No games found for this player".to_string();
        }
    }

    // Update Purchase History (Last Game Only)
    if let Ok(mut text) = purchase_history_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            let mut purchases: Vec<&super::types::PurchaseHistory> = game_state
                .purchase_history
                .iter()
                .filter_map(|((player, game_id, _), history)| {
                    if *player == current_player && *game_id == last_game_id {
                        Some(history)
                    } else {
                        None
                    }
                })
                .collect();

            if !purchases.is_empty() {
                let mut content = String::new();
                // Sort by orb type for consistent display
                purchases.sort_by_key(|p| format!("{:?}", p.orb_type));

                let total_purchases: u32 = purchases.iter().map(|p| p.purchase_count).sum();
                content.push_str(&format!("Total purchases: {}\n", total_purchases));
                content.push_str("Purchases by orb type:\n");

                for purchase in purchases {
                    content.push_str(&format!(
                        "  {:?}: {} times\n",
                        purchase.orb_type, purchase.purchase_count
                    ));
                }
                **text = content;
            } else {
                **text = format!("No purchase history for last game #{}", last_game_id);
            }
        } else {
            **text = "No games found for this player".to_string();
        }
    }
}

fn handle_start_game_button(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Visibility,
        ),
        (Changed<Interaction>, With<StartGameButton>),
    >,
    mut display_query: Query<
        &mut Visibility,
        (With<GameStateDisplayRoot>, Without<StartGameButton>),
    >,
) {
    for (interaction, mut bg_color, mut border_color, mut button_visibility) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgba(0.1, 0.2, 0.4, 0.95)); // Brighter space blue when pressed
                *border_color = BorderColor(Color::srgb(0.5, 0.8, 1.0)); // Brighter cyan border
                if let Ok(mut visibility) = display_query.single_mut() {
                    *visibility = Visibility::Visible;
                    *button_visibility = Visibility::Hidden; // Hide the button when UI is shown
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.08, 0.15, 0.35, 0.95)); // Slightly brighter on hover
                *border_color = BorderColor(Color::srgb(0.4, 0.7, 1.0)); // Glowing cyan border
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.05, 0.1, 0.25, 0.9)); // Default deep space blue
                *border_color = BorderColor(Color::srgb(0.3, 0.6, 1.0)); // Default cyan border
            }
        }
    }
}

fn handle_escape_button(
    mut escape_button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EscapeButton>),
    >,
    mut display_query: Query<&mut Visibility, (With<GameStateDisplayRoot>, Without<EscapeButton>)>,
    mut start_button_query: Query<
        &mut Visibility,
        (
            With<StartGameButton>,
            Without<GameStateDisplayRoot>,
            Without<EscapeButton>,
        ),
    >,
) {
    for (interaction, mut bg_color, mut border_color) in &mut escape_button_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgba(0.5, 0.2, 0.2, 0.95)); // Brighter red when pressed
                *border_color = BorderColor(Color::srgb(1.0, 0.5, 0.5)); // Brighter red border

                // Hide the game state UI
                if let Ok(mut display_visibility) = display_query.single_mut() {
                    *display_visibility = Visibility::Hidden;
                }

                // Show the start game button again
                if let Ok(mut start_button_visibility) = start_button_query.single_mut() {
                    *start_button_visibility = Visibility::Visible;
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.4, 0.15, 0.15, 0.95)); // Slightly brighter red on hover
                *border_color = BorderColor(Color::srgb(0.9, 0.4, 0.4)); // Glowing red border
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 0.9)); // Default dark red
                *border_color = BorderColor(Color::srgb(0.8, 0.3, 0.3)); // Default red border
            }
        }
    }
}

fn format_felt(felt: &Felt) -> String {
    let hex = format!("{:#x}", felt);
    if hex.len() > 10 {
        format!("{}...{}", &hex[0..6], &hex[hex.len() - 4..])
    } else {
        hex
    }
}
