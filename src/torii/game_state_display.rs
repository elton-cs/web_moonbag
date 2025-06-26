use bevy::prelude::*;
use bevy::ui::Val::*;
use starknet::core::types::Felt;
use std::collections::HashSet;

use super::game_state::GameState;

pub struct GameStateDisplayPlugin;

impl Plugin for GameStateDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerNavigation::default())
            .add_systems(
                Startup,
                (
                    spawn_game_state_display,
                    spawn_game_style_display,
                    spawn_start_game_button,
                ),
            )
            .add_systems(
                Update,
                (
                    collect_available_players,
                    update_game_state_display,
                    update_game_style_display,
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
struct GameStyleDisplay;

#[derive(Component)]
struct GameStyleDisplayRoot;

#[derive(Component)]
struct MoonRocksCountText;

#[derive(Component)]
struct HealthCountText;

#[derive(Component)]
struct LevelText;

#[derive(Component)]
struct PointsText;

#[derive(Component)]
struct BlueBarText;

#[derive(Component)]
struct OrangeBarText;

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

fn spawn_game_style_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root container (hidden by default)
    commands
        .spawn((
            GameStyleDisplayRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Percent(100.0),
                height: Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // Top left - MoonRocks crystal icon and count
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    top: Px(20.0),
                    left: Px(20.0),
                    width: Px(80.0),
                    height: Px(80.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    // Purple crystal placeholder
                    parent.spawn((
                        Node {
                            width: Px(60.0),
                            height: Px(60.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.6, 0.2, 0.8)), // Purple placeholder
                        BorderRadius::all(Px(8.0)),
                    ));

                    // MoonRocks count text
                    parent.spawn((
                        MoonRocksCountText,
                        Text::new("0"),
                        TextFont {
                            font: asset_server.load("fonts/font1.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Px(65.0),
                            left: Px(20.0),
                            ..default()
                        },
                    ));
                });

            // Top right - Health hearts
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    top: Px(20.0),
                    right: Px(20.0),
                    width: Px(100.0),
                    height: Px(80.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    // Health count text
                    parent.spawn((
                        HealthCountText,
                        Text::new("3"),
                        TextFont {
                            font: asset_server.load("fonts/font1.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.2, 0.2)), // Red for health
                    ));

                    // Red hearts placeholder
                    parent
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Px(5.0),
                            margin: UiRect::top(Px(5.0)),
                            ..default()
                        },))
                        .with_children(|parent| {
                            // Three heart placeholders
                            for _ in 0..3 {
                                parent.spawn((
                                    Node {
                                        width: Px(20.0),
                                        height: Px(20.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(1.0, 0.2, 0.2)), // Red hearts
                                    BorderRadius::all(Px(3.0)),
                                ));
                            }
                        });
                });

            // Top center - Level indicator (green oval)
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    top: Px(20.0),
                    left: Percent(50.0),
                    width: Px(80.0),
                    height: Px(40.0),
                    margin: UiRect::left(Px(-40.0)), // Center horizontally
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    // Green oval background
                    parent.spawn((
                        Node {
                            width: Px(80.0),
                            height: Px(40.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.8, 0.2)), // Green background
                        BorderRadius::all(Px(20.0)),                 // Oval shape
                    ));

                    // Level text
                    parent.spawn((
                        LevelText,
                        Text::new("1"),
                        TextFont {
                            font: asset_server.load("fonts/font1.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                    ));
                });

            // Center - Large moonbag with points
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    top: Percent(50.0),
                    left: Percent(50.0),
                    width: Px(200.0),
                    height: Px(200.0),
                    margin: UiRect {
                        left: Px(-100.0),
                        top: Px(-100.0),
                        ..default()
                    },
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    // Moonbag placeholder (large circle)
                    parent.spawn((
                        Node {
                            width: Px(180.0),
                            height: Px(180.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.8)), // Gray moonbag
                        BorderColor(Color::srgb(0.8, 0.8, 0.8)),
                        BorderRadius::all(Px(90.0)), // Circular
                    ));

                    // Points text inside moonbag
                    parent.spawn((
                        PointsText,
                        Text::new("0"),
                        TextFont {
                            font: asset_server.load("fonts/font1.otf"),
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                    ));
                });

            // Bottom status bar
            parent
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    bottom: Px(20.0),
                    left: Percent(50.0),
                    width: Px(300.0),
                    height: Px(50.0),
                    margin: UiRect::left(Px(-150.0)), // Center horizontally
                    flex_direction: FlexDirection::Row,
                    ..default()
                },))
                .with_children(|parent| {
                    // Blue section
                    parent
                        .spawn((
                            Node {
                                width: Px(120.0),
                                height: Px(50.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.4, 0.8)), // Blue background
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                BlueBarText,
                                Text::new("0"),
                                TextFont {
                                    font: asset_server.load("fonts/font1.otf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Orange section
                    parent
                        .spawn((
                            Node {
                                width: Px(120.0),
                                height: Px(50.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.8, 0.5, 0.2)), // Orange background
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                OrangeBarText,
                                Text::new("0"),
                                TextFont {
                                    font: asset_server.load("fonts/font1.otf"),
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Golden orb placeholder
                    parent.spawn((
                        Node {
                            width: Px(60.0),
                            height: Px(50.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.6, 0.2)), // Golden color
                        BorderRadius::all(Px(25.0)),
                    ));
                });

            // Escape button in top corner
            parent
                .spawn((
                    EscapeButton,
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        top: Px(20.0),
                        right: Px(140.0), // To the left of health hearts
                        width: Px(60.0),
                        height: Px(30.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 0.9)),
                    BorderColor(Color::srgb(0.8, 0.3, 0.3)),
                    BorderRadius::all(Px(5.0)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("ESC"),
                        TextFont {
                            font: asset_server.load("fonts/font1.otf"),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.8, 0.8)),
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
        (With<GameStyleDisplayRoot>, Without<StartGameButton>),
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

fn update_game_style_display(
    game_state: Option<Res<GameState>>,
    player_nav: Res<PlayerNavigation>,
    mut moonrocks_query: Query<&mut Text, With<MoonRocksCountText>>,
    mut health_query: Query<&mut Text, (With<HealthCountText>, Without<MoonRocksCountText>)>,
    mut level_query: Query<
        &mut Text,
        (
            With<LevelText>,
            Without<MoonRocksCountText>,
            Without<HealthCountText>,
        ),
    >,
    mut points_query: Query<
        &mut Text,
        (
            With<PointsText>,
            Without<MoonRocksCountText>,
            Without<HealthCountText>,
            Without<LevelText>,
        ),
    >,
    mut blue_bar_query: Query<
        &mut Text,
        (
            With<BlueBarText>,
            Without<MoonRocksCountText>,
            Without<HealthCountText>,
            Without<LevelText>,
            Without<PointsText>,
        ),
    >,
    mut orange_bar_query: Query<
        &mut Text,
        (
            With<OrangeBarText>,
            Without<MoonRocksCountText>,
            Without<HealthCountText>,
            Without<LevelText>,
            Without<PointsText>,
            Without<BlueBarText>,
        ),
    >,
) {
    let Some(game_state) = game_state else { return };
    let Some(current_player) = player_nav.get_current_player() else {
        // Set default values if no player selected
        if let Ok(mut text) = moonrocks_query.single_mut() {
            **text = "0".to_string();
        }
        if let Ok(mut text) = health_query.single_mut() {
            **text = "3".to_string();
        }
        if let Ok(mut text) = level_query.single_mut() {
            **text = "1".to_string();
        }
        if let Ok(mut text) = points_query.single_mut() {
            **text = "0".to_string();
        }
        if let Ok(mut text) = blue_bar_query.single_mut() {
            **text = "0".to_string();
        }
        if let Ok(mut text) = orange_bar_query.single_mut() {
            **text = "0".to_string();
        }
        return;
    };

    // Update MoonRocks count
    if let Ok(mut text) = moonrocks_query.single_mut() {
        if let Some(moon_rock) = game_state.moon_rocks.get(&current_player) {
            **text = moon_rock.amount.to_string();
        } else {
            **text = "0".to_string();
        }
    }

    // Get the last game for this player
    if let Some(last_game_id) = get_last_game_id(&game_state, &current_player) {
        if let Some(game) = game_state.games.get(&(current_player, last_game_id)) {
            // Update health
            if let Ok(mut text) = health_query.single_mut() {
                **text = game.health.to_string();
            }

            // Update level
            if let Ok(mut text) = level_query.single_mut() {
                **text = game.current_level.to_string();
            }

            // Update points
            if let Ok(mut text) = points_query.single_mut() {
                **text = game.points.to_string();
            }

            // For now, we'll use some game data for the blue and orange bars
            // Blue bar could be health * 100 or some other metric
            if let Ok(mut text) = blue_bar_query.single_mut() {
                **text = (game.health * 100).to_string();
            }

            // Orange bar could be level * 50 or some other metric
            if let Ok(mut text) = orange_bar_query.single_mut() {
                **text = (game.current_level * 50).to_string();
            }
        } else {
            // No game data, use defaults
            if let Ok(mut text) = health_query.single_mut() {
                **text = "3".to_string();
            }
            if let Ok(mut text) = level_query.single_mut() {
                **text = "1".to_string();
            }
            if let Ok(mut text) = points_query.single_mut() {
                **text = "0".to_string();
            }
            if let Ok(mut text) = blue_bar_query.single_mut() {
                **text = "0".to_string();
            }
            if let Ok(mut text) = orange_bar_query.single_mut() {
                **text = "0".to_string();
            }
        }
    }
}

fn handle_escape_button(
    mut escape_button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<EscapeButton>),
    >,
    mut visibility_query: Query<&mut Visibility>,
    game_style_display_query: Query<Entity, With<GameStyleDisplayRoot>>,
    start_button_query: Query<Entity, With<StartGameButton>>,
) {
    for (interaction, mut bg_color, mut border_color) in &mut escape_button_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgba(0.5, 0.2, 0.2, 0.95)); // Brighter red when pressed
                *border_color = BorderColor(Color::srgb(1.0, 0.5, 0.5)); // Brighter red border

                // Hide the game style display UI
                if let Ok(display_entity) = game_style_display_query.single() {
                    if let Ok(mut visibility) = visibility_query.get_mut(display_entity) {
                        *visibility = Visibility::Hidden;
                    }
                }

                // Show the start game button again
                if let Ok(start_button_entity) = start_button_query.single() {
                    if let Ok(mut visibility) = visibility_query.get_mut(start_button_entity) {
                        *visibility = Visibility::Visible;
                    }
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
