use bevy::prelude::*;
use crossbeam_channel::Receiver;
use std::sync::Arc;
use torii_client::Client;

#[derive(Resource, Clone)]
pub struct ToriiClient {
    pub client: Arc<Client>,
}

#[derive(Resource)]
pub(crate) struct ToriiClientTask(
    pub(crate) bevy::tasks::Task<Result<Client, Box<dyn std::error::Error + Send + Sync>>>,
);

#[derive(Event)]
pub struct EntityUpdateEvent {
    pub entity_id: String,
    pub update_data: String,
}

#[derive(Resource)]
pub(crate) struct EntityStreamReceiver(pub(crate) Receiver<EntityUpdateEvent>);

#[derive(Resource)]
pub(crate) struct EntityStreamTask(pub(crate) bevy::tasks::Task<()>);
