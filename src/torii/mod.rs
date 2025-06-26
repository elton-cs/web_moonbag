mod game_state;
mod resources;
mod systems;
mod types;
use bevy::prelude::*;
pub use game_state::GameState;
pub use resources::*;
pub use systems::*;
pub use types::*;

pub struct ToriiPlugin;

impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityUpdateEvent>()
            .add_systems(Startup, setup_torii_client)
            .add_systems(
                Update,
                (
                    poll_torii_client_task,
                    poll_initial_fetch_task,
                    poll_entity_stream_task,
                    log_entity_updates,
                ),
            );
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ToriiPlugin);
}
