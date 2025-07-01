//! Dojo v2 blockchain integration module.
//!
//! This module provides comprehensive blockchain integration for the game,
//! including transaction handling, entity updates, and type definitions.

pub mod plugin;
pub mod types;
pub mod events;
pub mod systems;
pub mod constants;

pub use plugin::DojoV2Plugin;
pub use types::*;
pub use events::*;
pub use systems::*;
pub use constants::*;

/// Export the plugin function for easy integration
pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(DojoV2Plugin);
}