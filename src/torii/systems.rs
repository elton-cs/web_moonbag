//! Systems for managing the Torii connection and syncing entities.

use crate::torii::{ToriiClient, ToriiConfig, ToriiConnectionState, client};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on},
};
use futures_lite::future;
use std::sync::Arc;

/// Component to track the async connection task
#[derive(Component)]
pub struct ToriiConnectionTask(Task<Result<torii_client::Client, String>>);

/// Initializes the Torii connection on startup.
pub fn initialize_torii_connection(
    mut commands: Commands,
    mut connection_state: ResMut<ToriiConnectionState>,
) {
    info!("Initializing Torii connection...");
    *connection_state = ToriiConnectionState::Connecting;

    let config = ToriiConfig::default();
    let task_pool = AsyncComputeTaskPool::get();

    // Spawn the async connection task using Bevy's task pool
    let task = task_pool.spawn(async move { client::create_client(config).await });

    // Spawn an entity with the task component to track progress
    commands.spawn(ToriiConnectionTask(task));
}

/// Checks the connection status and updates resources accordingly.
pub fn check_connection_status(
    mut commands: Commands,
    mut connection_state: ResMut<ToriiConnectionState>,
    current_client: Option<Res<ToriiClient>>,
    mut task_query: Query<(Entity, &mut ToriiConnectionTask)>,
) {
    // Skip if already connected
    if current_client.is_some() {
        return;
    }

    // Check if any connection tasks are complete
    for (entity, mut task) in &mut task_query {
        if let Some(result) = block_on(future::poll_once(&mut task.0)) {
            // Task is complete, remove the entity
            commands.entity(entity).despawn();

            match result {
                Ok(client) => {
                    commands.insert_resource(ToriiClient {
                        client: Arc::new(client),
                    });
                    *connection_state = ToriiConnectionState::Connected;
                    info!("Torii connection established successfully");
                }
                Err(error) => {
                    *connection_state = ToriiConnectionState::Failed(error.clone());
                    error!("Failed to connect to Torii: {}", error);
                }
            }
        }
    }
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
