mod client;
mod resources;
mod systems;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
pub use resources::*;
use std::time::Duration;

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
                    (
                        systems::spawn_entities_fetch_task.run_if(on_timer(Duration::from_secs(5))),
                        systems::check_entities_fetch_status,
                    )
                        .chain()
                        .run_if(resource_exists::<ToriiClient>),
                ),
            );
    }
}

// Legacy plugin function for backward compatibility
pub fn plugin(app: &mut App) {
    app.add_plugins(ToriiPlugin);
}
