//! Systems for managing the Torii connection and syncing entities.

use crate::torii::{ToriiClient, ToriiConfig, ToriiConnectionState, client};
use bevy::prelude::*;
use std::sync::Arc;

/// Initializes the Torii connection on startup.
pub fn initialize_torii_connection(mut connection_state: ResMut<ToriiConnectionState>) {
    info!("Initializing Torii connection...");
    *connection_state = ToriiConnectionState::Connecting;

    let config = ToriiConfig::default();

    // Spawn the async connection task
    wasm_bindgen_futures::spawn_local(async move {
        match client::create_client(config).await {
            Ok(client) => {
                // Store the client in a thread-local for later retrieval
                // This is a workaround for the async/sync boundary in WASM
                PENDING_CLIENT.with(|cell| {
                    *cell.borrow_mut() = Some(Arc::new(client));
                });
            }
            Err(e) => {
                error!("Failed to connect to Torii: {}", e);
                PENDING_ERROR.with(|cell| {
                    *cell.borrow_mut() = Some(e);
                });
            }
        }
    });
}

// Thread-local storage for passing the client from async to sync context
thread_local! {
    static PENDING_CLIENT: std::cell::RefCell<Option<Arc<torii_client::Client>>> = std::cell::RefCell::new(None);
    static PENDING_ERROR: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

/// Checks the connection status and updates resources accordingly.
pub fn check_connection_status(
    mut commands: Commands,
    mut connection_state: ResMut<ToriiConnectionState>,
    current_client: Option<Res<ToriiClient>>,
) {
    // Skip if already connected
    if current_client.is_some() {
        return;
    }

    // Check for pending client
    PENDING_CLIENT.with(|cell| {
        if let Some(client) = cell.borrow_mut().take() {
            commands.insert_resource(ToriiClient { client });
            *connection_state = ToriiConnectionState::Connected;
            info!("Torii connection established successfully");
        }
    });

    // Check for pending error
    PENDING_ERROR.with(|cell| {
        if let Some(error) = cell.borrow_mut().take() {
            *connection_state = ToriiConnectionState::Failed(error.clone());
            error!("Failed to connect to Torii: {}", error);
        }
    });
}

/// Syncs entities from Torii to the Bevy world.
pub fn sync_entities(_client: Res<ToriiClient>) {
    // TODO: Implement entity syncing logic
    // This would involve:
    // 1. Querying for updated entities from Torii
    // 2. Converting Torii entities to Bevy components
    // 3. Spawning/updating entities in the Bevy world

    // For now, this is a placeholder that runs periodically
    trace!("Syncing entities from Torii...");
}
