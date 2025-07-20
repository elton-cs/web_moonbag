//! UI styling constants and theme definitions for Dojo display using modern Bevy 0.16

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
