//! Example demonstrating how to use the ToriiPlugin and access connection state.

use bevy::prelude::*;
use web_moonbag::torii::{ToriiClient, ToriiConnectionState};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(web_moonbag::torii::plugin)
        .add_systems(Update, debug_torii_status)
        .run()
}

/// System that logs the current Torii connection status.
fn debug_torii_status(
    connection_state: Res<ToriiConnectionState>,
    torii_client: Option<Res<ToriiClient>>,
) {
    // Only log when state changes
    if connection_state.is_changed() {
        match connection_state.as_ref() {
            ToriiConnectionState::Disconnected => {
                info!("Torii: Disconnected");
            }
            ToriiConnectionState::Connecting => {
                info!("Torii: Connecting...");
            }
            ToriiConnectionState::Connected => {
                info!("Torii: Connected successfully!");
                if let Some(client) = torii_client {
                    info!("Torii client is available: {:?}", client.client.as_ref());
                }
            }
            ToriiConnectionState::Failed(error) => {
                error!("Torii: Connection failed - {}", error);
            }
        }
    }
}
