//! Simplified Dojo Display module for rendering blockchain game data in UI

use crate::torii::events::*;
use crate::torii::types::*;
use bevy::prelude::*;

/// Resource to hold current display data
#[derive(Resource, Default)]
pub struct DisplayData {
    pub current_game: Option<Game>,
    pub moon_rocks: Option<MoonRocks>,
}

/// Component for text displays
#[derive(Component)]
pub struct GameStatsText;

#[derive(Component)]
pub struct MoonRocksText;

#[derive(Component)]
pub struct PositionText;

pub struct DojoDisplayPlugin;

impl Plugin for DojoDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register events
            .add_event::<GameUpdatedEvent>()
            .add_event::<MoonRocksUpdatedEvent>()
            // Initialize resources
            .init_resource::<DisplayData>()
            // Setup systems
            .add_systems(Startup, setup_simple_ui)
            .add_systems(Update, (update_game_stats_simple, update_moon_rocks_simple));
    }
}

/// Setup simple UI display
fn setup_simple_ui(mut commands: Commands) {
    // Create simple text displays using modern Bevy 0.16 syntax
    commands.spawn((
        Text::new("Health: 0 | Points: 0 | Level: 1"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        GameStatsText,
    ));

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
        MoonRocksText,
    ));

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
        PositionText,
    ));
}

/// Update game stats display
fn update_game_stats_simple(
    mut events: EventReader<GameUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<GameStatsText>>,
) {
    for event in events.read() {
        display_data.current_game = Some(event.0.clone());
        let game = &event.0;

        for mut text in query.iter_mut() {
            *text = Text::new(format!(
                "Health: {} | Points: {} | Level: {}",
                game.health, game.points, game.current_level
            ));
        }
    }
}

/// Update moon rocks display
fn update_moon_rocks_simple(
    mut events: EventReader<MoonRocksUpdatedEvent>,
    mut display_data: ResMut<DisplayData>,
    mut query: Query<&mut Text, With<MoonRocksText>>,
) {
    for event in events.read() {
        display_data.moon_rocks = Some(event.0.clone());

        for mut text in query.iter_mut() {
            *text = Text::new(format!("Moon Rocks: {}", event.0.amount));
        }
    }
}
