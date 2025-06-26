use bevy::prelude::*;
use bevy::ui::Val::*;
use starknet::core::types::Felt;

use super::game_state::GameState;

pub struct GameStateDisplayPlugin;

impl Plugin for GameStateDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_state_display)
            .add_systems(Update, update_game_state_display)
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
                padding: UiRect::all(Px(20.0)),
                row_gap: Px(20.0),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            BorderRadius::all(Px(10.0)),
        )).with_children(|parent| {
            // Header
            parent.spawn((
                Text::new("Game State Display"),
                TextFont::from_font_size(32.0),
                TextColor(Color::WHITE),
                Node {
                    align_self: AlignSelf::Center,
                    margin: UiRect::bottom(Px(20.0)),
                    ..default()
                }
            ));

            // MoonRocks Section
            parent.spawn((
                Text::new("MoonRocks"),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            parent.spawn((
                MoonRocksText,
                Text::new("Loading..."),
                TextFont::from_font_size(16.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Px(20.0)),
                    ..default()
                }
            ));

            // Active Games Section
            parent.spawn((
                Text::new("Active Games"),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            parent.spawn((
                ActiveGamesText,
                Text::new("Loading..."),
                TextFont::from_font_size(16.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Px(20.0)),
                    ..default()
                }
            ));

            // Game Counters Section
            parent.spawn((
                Text::new("Game Counters"),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            parent.spawn((
                GameCountersText,
                Text::new("Loading..."),
                TextFont::from_font_size(16.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Px(20.0)),
                    ..default()
                }
            ));

            // Orb Bag Slots Section
            parent.spawn((
                Text::new("Orb Bag Slots Summary"),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            parent.spawn((
                OrbBagSlotsText,
                Text::new("Loading..."),
                TextFont::from_font_size(16.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Px(20.0)),
                    ..default()
                }
            ));

            // Shop Inventory Section
            parent.spawn((
                Text::new("Shop Inventory Summary"),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            parent.spawn((
                ShopInventoryText,
                Text::new("Loading..."),
                TextFont::from_font_size(16.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Px(20.0)),
                    ..default()
                }
            ));

            // Purchase History Section
            parent.spawn((
                Text::new("Purchase History Summary"),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            parent.spawn((
                PurchaseHistoryText,
                Text::new("Loading..."),
                TextFont::from_font_size(16.0),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
    });
}

fn update_game_state_display(
    game_state: Option<Res<GameState>>,
    mut moon_rocks_query: Query<&mut Text, With<MoonRocksText>>,
    mut active_games_query: Query<&mut Text, (With<ActiveGamesText>, Without<MoonRocksText>)>,
    mut game_counters_query: Query<&mut Text, (With<GameCountersText>, Without<MoonRocksText>, Without<ActiveGamesText>)>,
    mut orb_bag_slots_query: Query<&mut Text, (With<OrbBagSlotsText>, Without<MoonRocksText>, Without<ActiveGamesText>, Without<GameCountersText>)>,
    mut shop_inventory_query: Query<&mut Text, (With<ShopInventoryText>, Without<MoonRocksText>, Without<ActiveGamesText>, Without<GameCountersText>, Without<OrbBagSlotsText>)>,
    mut purchase_history_query: Query<&mut Text, (With<PurchaseHistoryText>, Without<MoonRocksText>, Without<ActiveGamesText>, Without<GameCountersText>, Without<OrbBagSlotsText>, Without<ShopInventoryText>)>,
) {
    let Some(game_state) = game_state else { return };

    // Update MoonRocks
    if let Ok(mut text) = moon_rocks_query.get_single_mut() {
        let mut content = String::new();
        for (_, moon_rock) in game_state.moon_rocks.iter() {
            content.push_str(&format!("Player: {} | Amount: {}\n", 
                format_felt(&moon_rock.player), moon_rock.amount));
        }
        if content.is_empty() {
            content = "No MoonRocks data".to_string();
        }
        **text = content;
    }

    // Update Active Games
    if let Ok(mut text) = active_games_query.get_single_mut() {
        let mut content = String::new();
        for (_, game) in game_state.games.iter() {
            if game.is_active {
                content.push_str(&format!(
                    "Player: {} | Game #{} | Health: {} | Points: {} | Level: {} | State: {:?}\n",
                    format_felt(&game.player), game.game_id, game.health, 
                    game.points, game.current_level, game.game_state
                ));
            }
        }
        if content.is_empty() {
            content = "No active games".to_string();
        }
        **text = content;
    }

    // Update Game Counters
    if let Ok(mut text) = game_counters_query.get_single_mut() {
        let mut content = String::new();
        for (_, counter) in game_state.game_counters.iter() {
            content.push_str(&format!("Player: {} | Next Game ID: {}\n", 
                format_felt(&counter.player), counter.next_game_id));
        }
        if content.is_empty() {
            content = "No game counter data".to_string();
        }
        **text = content;
    }

    // Update Orb Bag Slots
    if let Ok(mut text) = orb_bag_slots_query.get_single_mut() {
        let mut slots_by_game: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
        for ((_, game_id, _), _) in game_state.orb_bag_slots.iter() {
            *slots_by_game.entry(*game_id).or_insert(0) += 1;
        }
        
        let mut content = String::new();
        for (game_id, count) in slots_by_game.iter() {
            content.push_str(&format!("Game #{}: {} slots\n", game_id, count));
        }
        if content.is_empty() {
            content = "No orb bag slots data".to_string();
        }
        **text = content;
    }

    // Update Shop Inventory
    if let Ok(mut text) = shop_inventory_query.get_single_mut() {
        let mut items_by_game_level: std::collections::HashMap<(u32, u8), usize> = std::collections::HashMap::new();
        for ((_, game_id, level, _), _) in game_state.shop_inventory.iter() {
            *items_by_game_level.entry((*game_id, *level)).or_insert(0) += 1;
        }
        
        let mut content = String::new();
        for ((game_id, level), count) in items_by_game_level.iter() {
            content.push_str(&format!("Game #{} Level {}: {} items\n", game_id, level, count));
        }
        if content.is_empty() {
            content = "No shop inventory data".to_string();
        }
        **text = content;
    }

    // Update Purchase History
    if let Ok(mut text) = purchase_history_query.get_single_mut() {
        let mut aggregated: std::collections::HashMap<(Felt, u32), u32> = std::collections::HashMap::new();
        for ((player, game_id, _), history) in game_state.purchase_history.iter() {
            *aggregated.entry((*player, *game_id)).or_insert(0) += history.purchase_count;
        }
        
        let mut content = String::new();
        for ((player, game_id), total_count) in aggregated.iter() {
            content.push_str(&format!("Player: {} | Game #{}: {} purchases\n", 
                format_felt(player), game_id, total_count));
        }
        if content.is_empty() {
            content = "No purchase history data".to_string();
        }
        **text = content;
    }
}

fn toggle_display(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<GameStateDisplayRoot>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        if let Ok(mut visibility) = query.get_single_mut() {
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