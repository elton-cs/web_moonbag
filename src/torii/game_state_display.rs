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
                    update_health_hearts,
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

#[derive(Component)]
struct HealthHeart {
    index: usize,
}

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.1, 0.9)),
            Visibility::Hidden,
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            // Main game-style display container
            parent
                .spawn((
                    GameStateDisplay,
                    Node {
                        width: Percent(100.0),
                        height: Percent(100.0),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Navigation Header (Top)
                    parent
                        .spawn((Node {
                            position_type: PositionType::Absolute,
                            top: Px(20.0),
                            left: Percent(50.0),
                            width: Px(400.0),
                            height: Px(40.0),
                            margin: UiRect::left(Px(-200.0)), // Center horizontally
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
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
                                        height: Px(35.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: UiRect::all(Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.1, 0.3, 0.6, 0.8)),
                                    BorderColor(Color::srgb(0.3, 0.6, 1.0)),
                                    BorderRadius::all(Px(8.0)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("< PREV"),
                                        TextFont::from_font_size(14.0),
                                        TextColor(Color::srgb(0.8, 0.9, 1.0)),
                                    ));
                                });

                            // Player info
                            parent.spawn((
                                PlayerInfoText,
                                Text::new("No Players"),
                                TextFont::from_font_size(18.0),
                                TextColor(Color::srgb(0.9, 0.9, 1.0)),
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
                                        height: Px(35.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        border: UiRect::all(Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.1, 0.3, 0.6, 0.8)),
                                    BorderColor(Color::srgb(0.3, 0.6, 1.0)),
                                    BorderRadius::all(Px(8.0)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("NEXT >"),
                                        TextFont::from_font_size(14.0),
                                        TextColor(Color::srgb(0.8, 0.9, 1.0)),
                                    ));
                                });
                        });

                    // Escape Button (Top Right)
                    parent
                        .spawn((
                            EscapeButton,
                            Button,
                            Node {
                                position_type: PositionType::Absolute,
                                top: Px(20.0),
                                right: Px(20.0),
                                width: Px(80.0),
                                height: Px(35.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.6, 0.1, 0.1, 0.9)),
                            BorderColor(Color::srgb(1.0, 0.3, 0.3)),
                            BorderRadius::all(Px(8.0)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("ESC"),
                                TextFont::from_font_size(14.0),
                                TextColor(Color::srgb(1.0, 0.8, 0.8)),
                            ));
                        });

                    // Level Display (Top Center)
                    parent
                        .spawn((Node {
                            position_type: PositionType::Absolute,
                            top: Px(80.0),
                            right: Px(40.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Level 1"),
                                TextFont::from_font_size(20.0),
                                TextColor(Color::srgb(0.9, 0.9, 0.4)),
                            ));
                            parent.spawn((
                                GameCountersText,
                                Text::new("0"),
                                TextFont::from_font_size(32.0),
                                TextColor(Color::srgb(0.9, 0.9, 0.4)),
                            ));
                        });

                    // Central Moonbag
                    parent
                        .spawn((Node {
                            position_type: PositionType::Absolute,
                            top: Percent(35.0),
                            left: Percent(50.0),
                            width: Px(300.0),
                            height: Px(300.0),
                            margin: UiRect {
                                left: Px(-150.0), // Center horizontally
                                top: Px(-50.0),   // Adjust vertical position
                                ..default()
                            },
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Px(3.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.7)),
                        BorderColor(Color::srgb(0.4, 0.8, 1.0)),
                        BorderRadius::all(Px(150.0)), // Make it circular
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ActiveGamesText,
                                Text::new("12"),
                                TextFont::from_font_size(120.0),
                                TextColor(Color::srgb(0.9, 0.3, 0.9)),
                                Node {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                            ));
                        });

                    // Health Hearts (Top Left)
                    parent
                        .spawn((Node {
                            position_type: PositionType::Absolute,
                            top: Px(140.0),
                            right: Px(40.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Px(8.0),
                            ..default()
                        },))
                        .with_children(|parent| {
                            // Create 5 heart slots
                            for i in 0..5 {
                                parent.spawn((
                                    HealthHeart { index: i },
                                    Node {
                                        width: Px(30.0),
                                        height: Px(30.0),
                                        border: UiRect::all(Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.3, 0.1, 0.1)), // Start dimmed
                                    BorderColor(Color::srgb(0.5, 0.2, 0.2)),
                                    BorderRadius::all(Px(15.0)),
                                ));
                            }
                        });

                    // MoonRocks Display (Bottom Left)
                    parent
                        .spawn((Node {
                            position_type: PositionType::Absolute,
                            bottom: Px(40.0),
                            left: Px(40.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Px(10.0),
                            padding: UiRect::all(Px(12.0)),
                            border: UiRect::all(Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.2, 0.4, 0.8)),
                        BorderColor(Color::srgb(0.4, 0.7, 1.0)),
                        BorderRadius::all(Px(20.0)),
                        ))
                        .with_children(|parent| {
                            // Crystal icon placeholder
                            parent.spawn((
                                Node {
                                    width: Px(25.0),
                                    height: Px(25.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.5, 0.3, 0.9)),
                                BorderRadius::all(Px(6.0)),
                            ));
                            parent.spawn((
                                MoonRocksText,
                                Text::new("490"),
                                TextFont::from_font_size(24.0),
                                TextColor(Color::srgb(0.8, 0.9, 1.0)),
                            ));
                        });

                    // Cheddah Display (Bottom Right)
                    parent
                        .spawn((Node {
                            position_type: PositionType::Absolute,
                            bottom: Px(40.0),
                            right: Px(40.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Px(10.0),
                            padding: UiRect::all(Px(12.0)),
                            border: UiRect::all(Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.4, 0.2, 0.1, 0.8)),
                        BorderColor(Color::srgb(1.0, 0.6, 0.3)),
                        BorderRadius::all(Px(20.0)),
                        ))
                        .with_children(|parent| {
                            // Cheese icon placeholder
                            parent.spawn((
                                Node {
                                    width: Px(25.0),
                                    height: Px(25.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.7, 0.2)),
                                BorderRadius::all(Px(12.0)),
                            ));
                            parent.spawn((
                                ShopInventoryText,
                                Text::new("0"),
                                TextFont::from_font_size(24.0),
                                TextColor(Color::srgb(1.0, 0.9, 0.7)),
                            ));
                        });

                    // Hidden text elements for data (not displayed but used for updates)
                    parent.spawn((
                        OrbBagSlotsText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(1.0), // Hidden
                        TextColor(Color::NONE),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Px(-1000.0), // Off-screen
                            ..default()
                        },
                    ));
                    parent.spawn((
                        PurchaseHistoryText,
                        Text::new("Loading..."),
                        TextFont::from_font_size(1.0), // Hidden
                        TextColor(Color::NONE),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Px(-1000.0), // Off-screen
                            ..default()
                        },
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
            **text = "0".to_string();
        }
        if let Ok(mut text) = active_games_query.single_mut() {
            **text = "0".to_string();
        }
        if let Ok(mut text) = game_counters_query.single_mut() {
            **text = "0".to_string();
        }
        if let Ok(mut text) = shop_inventory_query.single_mut() {
            **text = "0".to_string();
        }
        return;
    };

    // Update MoonRocks display (bottom left currency)
    if let Ok(mut text) = moon_rocks_query.single_mut() {
        if let Some(moon_rock) = game_state.moon_rocks.get(&current_player) {
            **text = moon_rock.amount.to_string();
        } else {
            **text = "0".to_string();
        }
    }

    // Update Points display (central moonbag number)
    if let Ok(mut text) = active_games_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            if let Some(game) = game_state.games.get(&(current_player, last_game_id)) {
                **text = game.points.to_string();
            } else {
                **text = "0".to_string();
            }
        } else {
            **text = "0".to_string();
        }
    }

    // Update Level display (top right)
    if let Ok(mut text) = game_counters_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            if let Some(game) = game_state.games.get(&(current_player, last_game_id)) {
                **text = game.current_level.to_string();
            } else {
                **text = "1".to_string();
            }
        } else {
            **text = "1".to_string();
        }
    }

    // Update Cheddah display (bottom right currency)
    if let Ok(mut text) = shop_inventory_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            if let Some(game) = game_state.games.get(&(current_player, last_game_id)) {
                **text = game.cheddah.to_string();
            } else {
                **text = "0".to_string();
            }
        } else {
            **text = "0".to_string();
        }
    }

    // Hidden elements - still updated for potential future use
    if let Ok(mut text) = orb_bag_slots_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            let slots_count = game_state
                .orb_bag_slots
                .iter()
                .filter(|((player, game_id, _), _)| {
                    *player == current_player && *game_id == last_game_id
                })
                .count();
            **text = format!("Orb slots: {}", slots_count);
        } else {
            **text = "No orb slots".to_string();
        }
    }

    if let Ok(mut text) = purchase_history_query.single_mut() {
        if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
            let total_purchases: u32 = game_state
                .purchase_history
                .iter()
                .filter_map(|((player, game_id, _), history)| {
                    if *player == current_player && *game_id == last_game_id {
                        Some(history.purchase_count)
                    } else {
                        None
                    }
                })
                .sum();
            **text = format!("Total purchases: {}", total_purchases);
        } else {
            **text = "No purchases".to_string();
        }
    }
}

fn update_health_hearts(
    game_state: Option<Res<GameState>>,
    player_nav: Res<PlayerNavigation>,
    mut hearts_query: Query<(&HealthHeart, &mut BackgroundColor, &mut BorderColor)>,
) {
    let Some(game_state) = game_state else { return };
    let Some(current_player) = player_nav.get_current_player() else { 
        // No player selected - dim all hearts
        for (_, mut bg_color, mut border_color) in &mut hearts_query {
            *bg_color = BackgroundColor(Color::srgb(0.3, 0.1, 0.1));
            *border_color = BorderColor(Color::srgb(0.5, 0.2, 0.2));
        }
        return; 
    };

    let current_health = if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
        if let Some(game) = game_state.games.get(&(current_player, last_game_id)) {
            game.health as usize
        } else {
            5 // Default health
        }
    } else {
        5 // Default health
    };

    // Update heart appearances based on current health
    for (heart, mut bg_color, mut border_color) in &mut hearts_query {
        if heart.index < current_health {
            // Active heart - bright red
            *bg_color = BackgroundColor(Color::srgb(0.9, 0.2, 0.3));
            *border_color = BorderColor(Color::srgb(1.0, 0.4, 0.5));
        } else {
            // Inactive heart - dimmed
            *bg_color = BackgroundColor(Color::srgb(0.3, 0.1, 0.1));
            *border_color = BorderColor(Color::srgb(0.5, 0.2, 0.2));
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
