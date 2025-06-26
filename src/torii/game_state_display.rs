use bevy::prelude::*;
use bevy::ui::Val::*;
use starknet::core::types::Felt;
use std::collections::HashSet;

use super::game_state::GameState;

pub struct GameStateDisplayPlugin;

impl Plugin for GameStateDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerNavigation::default())
            .add_systems(Startup, spawn_game_state_display)
            .add_systems(Update, (
                collect_available_players,
                update_game_state_display,
                handle_navigation_buttons,
            ))
            .add_systems(Update, toggle_display);
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
    commands.spawn((
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
    )).with_children(|parent| {
        // Main display container
        parent.spawn((
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
        )).with_children(|parent| {
            // Navigation Header
            parent.spawn((
                Node {
                    width: Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::bottom(Px(10.0)),
                    ..default()
                },
            )).with_children(|parent| {
                // Previous button
                parent.spawn((
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
                )).with_children(|parent| {
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
                    }
                ));
                
                // Next button
                parent.spawn((
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
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Next >"),
                        TextFont::from_font_size(12.0),
                        TextColor(Color::WHITE),
                    ));
                });
            });
            
            // Title
            parent.spawn((
                Text::new("Player Game Data"),
                TextFont::from_font_size(20.0),
                TextColor(Color::WHITE),
                Node {
                    align_self: AlignSelf::Center,
                    margin: UiRect::bottom(Px(15.0)),
                    ..default()
                }
            ));

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
                }
            ));

            // Active Games Section
            parent.spawn((
                Text::new("Active Games"),
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
                }
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
                }
            ));

            // Orb Bag Slots Section
            parent.spawn((
                Text::new("Orb Bag Slots Summary"),
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
                }
            ));

            // Shop Inventory Section
            parent.spawn((
                Text::new("Shop Inventory Summary"),
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
                }
            ));

            // Purchase History Section
            parent.spawn((
                Text::new("Purchase History Summary"),
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
            **text = format!("Player {} ({}/{})", 
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
        (Changed<Interaction>, With<PrevPlayerButton>)
    >,
    mut next_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NextPlayerButton>, Without<PrevPlayerButton>)
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

fn update_game_state_display(
    game_state: Option<Res<GameState>>,
    player_nav: Res<PlayerNavigation>,
    mut moon_rocks_query: Query<&mut Text, With<MoonRocksText>>,
    mut active_games_query: Query<&mut Text, (With<ActiveGamesText>, Without<MoonRocksText>)>,
    mut game_counters_query: Query<&mut Text, (With<GameCountersText>, Without<MoonRocksText>, Without<ActiveGamesText>)>,
    mut orb_bag_slots_query: Query<&mut Text, (With<OrbBagSlotsText>, Without<MoonRocksText>, Without<ActiveGamesText>, Without<GameCountersText>)>,
    mut shop_inventory_query: Query<&mut Text, (With<ShopInventoryText>, Without<MoonRocksText>, Without<ActiveGamesText>, Without<GameCountersText>, Without<OrbBagSlotsText>)>,
    mut purchase_history_query: Query<&mut Text, (With<PurchaseHistoryText>, Without<MoonRocksText>, Without<ActiveGamesText>, Without<GameCountersText>, Without<OrbBagSlotsText>, Without<ShopInventoryText>)>,
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

    // Update Active Games
    if let Ok(mut text) = active_games_query.single_mut() {
        let mut content = String::new();
        for ((player, game_id), game) in game_state.games.iter() {
            if *player == current_player && game.is_active {
                content.push_str(&format!(
                    "Game #{} | Health: {} | Points: {} | Level: {} | State: {:?}\n",
                    game_id, game.health, game.points, game.current_level, game.game_state
                ));
            }
        }
        if content.is_empty() {
            content = "No active games for this player".to_string();
        }
        **text = content;
    }

    // Update Game Counters
    if let Ok(mut text) = game_counters_query.single_mut() {
        if let Some(counter) = game_state.game_counters.get(&current_player) {
            **text = format!("Next Game ID: {}", counter.next_game_id);
        } else {
            **text = "No game counter data for this player".to_string();
        }
    }

    // Update Orb Bag Slots
    if let Ok(mut text) = orb_bag_slots_query.single_mut() {
        let mut slots_by_game: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
        for ((player, game_id, _), _) in game_state.orb_bag_slots.iter() {
            if *player == current_player {
                *slots_by_game.entry(*game_id).or_insert(0) += 1;
            }
        }
        
        let mut content = String::new();
        for (game_id, count) in slots_by_game.iter() {
            content.push_str(&format!("Game #{}: {} slots\n", game_id, count));
        }
        if content.is_empty() {
            content = "No orb bag slots data for this player".to_string();
        }
        **text = content;
    }

    // Update Shop Inventory
    if let Ok(mut text) = shop_inventory_query.single_mut() {
        let mut items_by_game_level: std::collections::HashMap<(u32, u8), usize> = std::collections::HashMap::new();
        for ((player, game_id, level, _), _) in game_state.shop_inventory.iter() {
            if *player == current_player {
                *items_by_game_level.entry((*game_id, *level)).or_insert(0) += 1;
            }
        }
        
        let mut content = String::new();
        for ((game_id, level), count) in items_by_game_level.iter() {
            content.push_str(&format!("Game #{} Level {}: {} items\n", game_id, level, count));
        }
        if content.is_empty() {
            content = "No shop inventory data for this player".to_string();
        }
        **text = content;
    }

    // Update Purchase History
    if let Ok(mut text) = purchase_history_query.single_mut() {
        let mut aggregated: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for ((player, game_id, _), history) in game_state.purchase_history.iter() {
            if *player == current_player {
                *aggregated.entry(*game_id).or_insert(0) += history.purchase_count;
            }
        }
        
        let mut content = String::new();
        for (game_id, total_count) in aggregated.iter() {
            content.push_str(&format!("Game #{}: {} purchases\n", game_id, total_count));
        }
        if content.is_empty() {
            content = "No purchase history data for this player".to_string();
        }
        **text = content;
    }
}

fn toggle_display(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<GameStateDisplayRoot>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        if let Ok(mut visibility) = query.single_mut() {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

fn format_felt(felt: &Felt) -> String {
    let hex = format!("{:#x}", felt);
    if hex.len() > 10 {
        format!("{}...{}", &hex[0..6], &hex[hex.len()-4..])
    } else {
        hex
    }
}