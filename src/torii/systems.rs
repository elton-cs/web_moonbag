//! Systems for managing the Torii connection and syncing entities.

use crate::torii::{ToriiClient, ToriiConfig, ToriiConnectionState, client};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on},
};
use futures_lite::future;
use std::sync::Arc;
use torii_client::error::Error;

/// Component to track the async connection task
#[derive(Component)]
pub struct ToriiConnectionTask(Task<Result<torii_client::Client, Error>>);

/// Component to track the async entities fetch task
#[derive(Component)]
pub struct ToriiEntitiesFetchTask(Task<Result<String, Error>>);

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
                    error!("Failed to connect to Torii: {}", error);
                    *connection_state = ToriiConnectionState::Failed(error);
                }
            }
        }
    }
}

/// Spawns a task to fetch entities from Torii
pub fn spawn_entities_fetch_task(
    mut commands: Commands,
    client: Res<ToriiClient>,
    query: Query<&ToriiEntitiesFetchTask>,
) {
    // Only spawn a new task if there isn't one already running
    if !query.is_empty() {
        return;
    }

    trace!("Spawning task to fetch entities from Torii...");

    let task_pool = AsyncComputeTaskPool::get();
    let client_clone = client.client.clone();

    // Spawn the async task to fetch entities or metadata
    let task = task_pool.spawn(async move {
        // For now, let's fetch metadata as a demonstration
        // In a real implementation, you would use the proper Query type
        match client_clone.metadata().await {
            Ok(metadata) => {
                // Return metadata as a string for logging
                Ok(format!(
                    "World Address: {}, Number of Models: {}",
                    metadata.world_address,
                    metadata.models.len()
                ))
            }
            Err(e) => Err(e),
        }
    });

    // Spawn an entity with the task component to track progress
    commands.spawn(ToriiEntitiesFetchTask(task));
}

/// Checks entity fetch task completion and logs the results
pub fn check_entities_fetch_status(
    mut commands: Commands,
    mut task_query: Query<(Entity, &mut ToriiEntitiesFetchTask)>,
) {
    for (entity, mut task) in &mut task_query {
        if let Some(result) = block_on(future::poll_once(&mut task.0)) {
            // Task is complete, remove the entity
            commands.entity(entity).despawn();

            match result {
                Ok(metadata_info) => {
                    info!("Successfully fetched data from Torii: {}", metadata_info);
                }
                Err(error) => {
                    error!("Failed to fetch data from Torii: {}", error);
                }
            }
        }
    }
}
