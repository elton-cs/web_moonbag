use bevy::{
    prelude::*,
    tasks::{IoTaskPool, futures_lite::StreamExt},
};
use starknet::core::types::Felt;
use std::sync::Arc;
use torii_client::Client;

#[derive(Resource, Clone)]
pub struct ToriiClient {
    pub client: Arc<Client>,
}

#[derive(Resource)]
struct ToriiClientTask(bevy::tasks::Task<Result<Client, Box<dyn std::error::Error + Send + Sync>>>);

#[derive(Event)]
pub struct EntityUpdateEvent {
    pub entity_id: String,
    pub update_data: String,
}

#[derive(Resource)]
struct EntityStreamTask(
    bevy::tasks::Task<Result<EntityUpdateEvent, Box<dyn std::error::Error + Send + Sync>>>,
);

pub struct ToriiPlugin;

impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityUpdateEvent>()
            .add_systems(Startup, setup_torii_client)
            .add_systems(
                Update,
                (
                    poll_torii_client_task,
                    poll_entity_stream_task,
                    log_entity_updates,
                )
                    .chain(),
            );
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ToriiPlugin);
}

async fn create_torii_client() -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let torii_url = "https://api.cartridge.gg/x/moonbagvibes/torii";
    let world_address = "0x04d9778a74d2c9e6e7e4a24cbe913998a80de217c66ee173a604d06dea5469c3";
    let world_felt = Felt::from_hex_unchecked(world_address);

    let client = Client::new(torii_url.to_string(), world_felt).await?;
    Ok(client)
}

fn setup_torii_client(mut commands: Commands) {
    info!("Starting Torii client creation...");
    let task_pool = IoTaskPool::get();
    let task = task_pool.spawn(create_torii_client());
    commands.insert_resource(ToriiClientTask(task));
}

fn poll_torii_client_task(mut commands: Commands, task_res: Option<ResMut<ToriiClientTask>>) {
    if let Some(mut task_res) = task_res {
        if let Some(result) = bevy::tasks::block_on(bevy::tasks::poll_once(&mut task_res.0)) {
            match result {
                Ok(client) => {
                    info!("Torii client created successfully");
                    let client_arc = Arc::new(client);
                    commands.insert_resource(ToriiClient {
                        client: client_arc.clone(),
                    });

                    // Start the entity stream immediately
                    info!("Starting continuous entity stream...");
                    let task_pool = IoTaskPool::get();
                    let task = task_pool.spawn(create_entity_stream(client_arc));
                    commands.insert_resource(EntityStreamTask(task));
                }
                Err(e) => {
                    error!("Failed to create Torii client: {}", e);
                }
            }
            commands.remove_resource::<ToriiClientTask>();
        }
    }
}

async fn create_entity_stream(
    client: Arc<Client>,
) -> Result<EntityUpdateEvent, Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting entity update stream...");
    let mut stream = client.on_entity_updated(None).await?;

    // Wait for the next update from the stream
    if let Some(entity_update) = stream.next().await {
        match entity_update {
            Ok(update) => {
                info!("Entity update received: {:?}", update);

                // Extract useful information from the update
                let event = EntityUpdateEvent {
                    entity_id: format!("entity_{}", update.0),
                    update_data: format!("{:?}", update),
                };

                Ok(event)
            }
            Err(e) => {
                error!("Error in entity stream: {:?}", e);
                Err(e.into())
            }
        }
    } else {
        Err("Stream ended unexpectedly".into())
    }
}

fn poll_entity_stream_task(
    mut commands: Commands,
    task_res: Option<ResMut<EntityStreamTask>>,
    mut event_writer: EventWriter<EntityUpdateEvent>,
    client_res: Option<Res<ToriiClient>>,
) {
    if let Some(mut task_res) = task_res {
        if let Some(result) = bevy::tasks::block_on(bevy::tasks::poll_once(&mut task_res.0)) {
            match result {
                Ok(event) => {
                    info!("Entity stream received update");
                    event_writer.write(event);

                    // Immediately start waiting for the next update
                    if let Some(client_res) = client_res {
                        let task_pool = IoTaskPool::get();
                        let task = task_pool.spawn(create_entity_stream(client_res.client.clone()));
                        commands.insert_resource(EntityStreamTask(task));
                    }
                }
                Err(e) => {
                    error!("Entity stream failed: {:?}", e);

                    // Restart the stream after an error
                    if let Some(client_res) = client_res {
                        info!("Restarting entity stream after error...");
                        let task_pool = IoTaskPool::get();
                        let task = task_pool.spawn(create_entity_stream(client_res.client.clone()));
                        commands.insert_resource(EntityStreamTask(task));
                    }
                }
            }
        }
    }
}

fn log_entity_updates(mut event_reader: EventReader<EntityUpdateEvent>) {
    for event in event_reader.read() {
        info!(
            "🔄 Entity Update - ID: {}, Data: {}",
            event.entity_id, event.update_data
        );
    }
}
