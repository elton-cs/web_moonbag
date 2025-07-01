//! UI styling constants and theme definitions for Dojo display

use bevy::prelude::*;

/// Main colors for the UI theme
pub struct Colors;

impl Colors {
    pub const BACKGROUND: Color = Color::srgba(0.1, 0.1, 0.1, 0.9);
    pub const PANEL_BACKGROUND: Color = Color::srgba(0.2, 0.2, 0.2, 0.8);
    pub const TEXT_PRIMARY: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
    pub const TEXT_SECONDARY: Color = Color::srgba(0.7, 0.7, 0.7, 1.0);
    pub const ACCENT_BLUE: Color = Color::srgba(0.3, 0.6, 1.0, 1.0);
    pub const ACCENT_GREEN: Color = Color::srgba(0.3, 1.0, 0.3, 1.0);
    pub const ACCENT_RED: Color = Color::srgba(1.0, 0.3, 0.3, 1.0);
    pub const ACCENT_YELLOW: Color = Color::srgba(1.0, 0.9, 0.2, 1.0);
    pub const BORDER: Color = Color::srgba(0.4, 0.4, 0.4, 1.0);
}

/// Typography settings
pub struct Typography;

impl Typography {
    pub const TITLE_SIZE: f32 = 20.0;
    pub const HEADER_SIZE: f32 = 16.0;
    pub const BODY_SIZE: f32 = 14.0;
    pub const SMALL_SIZE: f32 = 12.0;
}

/// Layout spacing constants
pub struct Spacing;

impl Spacing {
    pub const SMALL: f32 = 4.0;
    pub const MEDIUM: f32 = 8.0;
    pub const LARGE: f32 = 16.0;
    pub const XLARGE: f32 = 24.0;
}

/// Common style builders for Node components
pub struct StyleBuilders;

impl StyleBuilders {
    /// Create a panel container style
    pub fn panel_container() -> Node {
        Node {
            padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
            margin: UiRect::all(Val::Px(Spacing::SMALL)),
            border: UiRect::all(Val::Px(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::FlexStart,
            ..default()
        }
    }

    /// Create a row container style
    pub fn row_container() -> Node {
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            margin: UiRect::vertical(Val::Px(Spacing::SMALL)),
            ..default()
        }
    }

    /// Create the main display root style
    pub fn display_root() -> Node {
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            width: Val::Px(400.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            ..default()
        }
    }

    /// Create orb bag grid style
    pub fn orb_bag_grid() -> Node {
        Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
            column_gap: Val::Px(Spacing::SMALL),
            row_gap: Val::Px(Spacing::SMALL),
            padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
            ..default()
        }
    }

    /// Create orb slot style
    pub fn orb_slot() -> Node {
        Node {
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            border: UiRect::all(Val::Px(1.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }
    }
}