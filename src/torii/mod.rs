mod client;
mod resources;
mod systems;
use bevy::prelude::*;
pub use resources::*;

pub struct ToriiPlugin;

impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<ToriiConnectionState>()
            // Add systems
            .add_systems(Startup, systems::initialize_torii_connection)
            .add_systems(
                Update,
                (
                    systems::check_connection_status,
                    systems::sync_entities.run_if(resource_exists::<ToriiClient>),
                ),
            );
    }
}

// Legacy plugin function for backward compatibility
pub fn plugin(app: &mut App) {
    app.add_plugins(ToriiPlugin);
}
