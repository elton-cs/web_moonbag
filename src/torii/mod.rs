//! Dojo v2 blockchain integration module.
//!
//! This module provides comprehensive blockchain integration for the game,
//! including transaction handling, entity updates, and type definitions.

pub mod constants;
pub mod events;
pub mod plugin;
pub mod systems;
pub mod types;

pub use constants::*;
pub use events::*;
pub use plugin::DojoV2Plugin;
pub use systems::*;
pub use types::*;

/// Export the plugin function for easy integration
pub fn plugin(app: &mut bevy::prelude::App) {
    app.add_plugins(DojoV2Plugin);
}
