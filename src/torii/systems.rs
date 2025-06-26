use bevy::{
    prelude::*,
    tasks::{IoTaskPool, futures_lite::StreamExt},
};
use crossbeam_channel::Sender;
use starknet::core::types::Felt;
use std::sync::Arc;
use torii_client::Client;

use super::resources::*;
use super::types::{DojoModel, convert_dojo_struct};

pub async fn create_torii_client() -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let torii_url = "https://api.cartridge.gg/x/moonbagvibes/torii";
    let world_address = "0x04d9778a74d2c9e6e7e4a24cbe913998a80de217c66ee173a604d06dea5469c3";
    let world_felt = Felt::from_hex_unchecked(world_address);

    let client = Client::new(torii_url.to_string(), world_felt).await?;
    Ok(client)
}

pub fn setup_torii_client(mut commands: Commands) {
    info!("Starting Torii client creation...");
    let task_pool = IoTaskPool::get();
    let task = task_pool.spawn(create_torii_client());
    commands.insert_resource(ToriiClientTask(task));
}

pub fn poll_torii_client_task(mut commands: Commands, task_res: Option<ResMut<ToriiClientTask>>) {
    if let Some(mut task_res) = task_res {
        if let Some(result) = bevy::tasks::block_on(bevy::tasks::poll_once(&mut task_res.0)) {
            match result {
                Ok(client) => {
                    info!("Torii client created successfully");
                    let client_arc = Arc::new(client);
                    commands.insert_resource(ToriiClient {
                        client: client_arc.clone(),
                    });

                    start_entity_stream(&mut commands, client_arc);
                }
                Err(e) => {
                    error!("Failed to create Torii client: {}", e);
                }
            }
            commands.remove_resource::<ToriiClientTask>();
        }
    }
}

fn start_entity_stream(commands: &mut Commands, client_arc: Arc<Client>) {
    info!("Starting continuous entity stream...");
    let (sender, receiver) = crossbeam_channel::unbounded();
    commands.insert_resource(EntityStreamReceiver(receiver));

    let task_pool = IoTaskPool::get();
    let task = task_pool.spawn(run_persistent_entity_stream(client_arc, sender));
    commands.insert_resource(EntityStreamTask(task));
}

pub async fn run_persistent_entity_stream(client: Arc<Client>, sender: Sender<EntityUpdateEvent>) {
    info!("Starting persistent entity update stream...");

    loop {
        match client.on_entity_updated(None).await {
            Ok(mut stream) => {
                info!("Entity stream connected, waiting for updates...");

                while let Some(entity_update) = stream.next().await {
                    match entity_update {
                        Ok((timestamp, entity)) => {
                            // update is (u64, torii_proto::schema::Entity) tuple
                            debug!("Entity update at timestamp: {}", timestamp);
                            let entity_id = format!("{:#x}", entity.hashed_keys);

                            // Process each model in the entity
                            let mut model_opt = None;
                            let mut model_info = String::new();

                            for model in &entity.models {
                                match convert_dojo_struct(model) {
                                    Ok(converted_model) => {
                                        model_info = format!("{:?}", converted_model);
                                        model_opt = Some(converted_model);
                                        break; // Use the first successfully converted model
                                    }
                                    Err(e) => {
                                        model_info =
                                            format!("Unknown model: {} ({:?})", model.name, e);
                                    }
                                }
                            }

                            if model_info.is_empty() {
                                model_info = "No models found".to_string();
                            }

                            let event = EntityUpdateEvent {
                                entity_id,
                                update_data: model_info,
                                model: model_opt,
                            };

                            if sender.send(event).is_err() {
                                warn!("Entity stream receiver dropped, stopping stream");
                                return;
                            }
                        }
                        Err(e) => {
                            error!("Error in entity stream: {:?}", e);
                            break;
                        }
                    }
                }

                error!("Entity stream ended, reconnecting...");
            }
            Err(e) => {
                error!("Failed to create entity stream: {:?}", e);
            }
        }

        bevy::tasks::futures_lite::future::yield_now().await;
    }
}

pub fn poll_entity_stream_task(
    receiver_res: Option<Res<EntityStreamReceiver>>,
    mut event_writer: EventWriter<EntityUpdateEvent>,
) {
    if let Some(receiver_res) = receiver_res {
        while let Ok(event) = receiver_res.0.try_recv() {
            event_writer.write(event);
        }
    }
}

pub fn log_entity_updates(mut event_reader: EventReader<EntityUpdateEvent>) {
    for event in event_reader.read() {
        match &event.model {
            Some(DojoModel::MoonRocks(moon_rocks)) => {
                info!(
                    "💎 MoonRocks: player {:#x} has {} rocks",
                    moon_rocks.player, moon_rocks.amount
                );
            }
            Some(DojoModel::Game(game)) => {
                info!(
                    "🎮 Game #{}: level {}, health: {}, points: {}, state: {:?}",
                    game.game_id, game.current_level, game.health, game.points, game.game_state
                );
            }
            Some(DojoModel::GameCounter(counter)) => {
                info!(
                    "🔢 Next game ID for player {:#x}: {}",
                    counter.player, counter.next_game_id
                );
            }
            Some(DojoModel::ActiveGame(active)) => {
                info!(
                    "🎯 Active game #{} for player {:#x}",
                    active.game_id, active.player
                );
            }
            Some(DojoModel::OrbBagSlot(slot)) => {
                info!(
                    "🎱 Orb slot [{}]: {:?} (active: {})",
                    slot.slot_index, slot.orb_type, slot.is_active
                );
            }
            Some(DojoModel::DrawnOrb(drawn)) => {
                info!(
                    "🎲 Drew {:?} orb at position {}",
                    drawn.orb_type, drawn.draw_index
                );
            }
            Some(DojoModel::ShopInventory(shop)) => {
                info!(
                    "🛍️ Shop slot {}: {:?} orb ({}🧀, {:?})",
                    shop.slot_index, shop.orb_type, shop.base_price, shop.rarity
                );
            }
            Some(DojoModel::PurchaseHistory(purchase)) => {
                info!(
                    "💰 Purchased {:?} orb {} times",
                    purchase.orb_type, purchase.purchase_count
                );
            }
            None => {
                debug!(
                    "❓ Unknown model update for entity {}: {}",
                    event.entity_id, event.update_data
                );
            }
        }
    }
}
