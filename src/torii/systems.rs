use bevy::{
    prelude::*,
    tasks::{IoTaskPool, futures_lite::StreamExt},
};
use crossbeam_channel::Sender;
use starknet::core::types::Felt;
use std::sync::Arc;
use torii_client::Client;
use torii_proto::Query;

use super::resources::*;
use super::types::{DojoModel, convert_dojo_struct};
use super::game_state::GameState;

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
                    
                    // Initialize game state
                    commands.insert_resource(GameState::new());

                    // Start fetching initial entities
                    fetch_initial_entities(&mut commands, client_arc.clone());
                    
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
    mut game_state: Option<ResMut<GameState>>,
) {
    if let Some(receiver_res) = receiver_res {
        while let Ok(event) = receiver_res.0.try_recv() {
            // Update game state if available
            if let Some(ref mut state) = game_state {
                if let Some(ref model) = event.model {
                    state.update_from_model(model);
                }
            }
            
            event_writer.write(event);
        }
    }
}

fn fetch_initial_entities(commands: &mut Commands, client_arc: Arc<Client>) {
    info!("Starting initial entity fetch...");
    let task_pool = IoTaskPool::get();
    let task = task_pool.spawn(fetch_all_entities(client_arc));
    commands.insert_resource(InitialFetchTask(task));
}

#[derive(Resource)]
pub(crate) struct InitialFetchTask(
    pub(crate) bevy::tasks::Task<Result<Vec<DojoModel>, Box<dyn std::error::Error + Send + Sync>>>,
);

pub async fn fetch_all_entities(
    client: Arc<Client>,
) -> Result<Vec<DojoModel>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Fetching all entities from Torii...");
    
    let mut all_models = Vec::new();
    let mut cursor = None;
    
    loop {
        // Create a query to fetch all entities
        let query = Query {
            clause: None,
            pagination: torii_proto::Pagination {
                cursor: cursor.clone(),
                limit: 100,
                direction: torii_proto::PaginationDirection::Forward,
                order_by: vec![],
            },
            no_hashed_keys: false,
            models: vec![],
            historical: false,
        };
        
        let page = client.entities(query).await?;
        
        // Process entities from this page
        for entity in &page.items {
            for model in &entity.models {
                match convert_dojo_struct(model) {
                    Ok(converted_model) => {
                        all_models.push(converted_model);
                    }
                    Err(e) => {
                        warn!("Failed to convert model {}: {:?}", model.name, e);
                    }
                }
            }
        }
        
        // Check if we need to fetch more pages
        if page.next_cursor.is_none() {
            break;
        }
        
        cursor = page.next_cursor;
    }
    
    info!("Fetched {} models from initial query", all_models.len());
    Ok(all_models)
}

pub fn poll_initial_fetch_task(
    mut commands: Commands,
    task_res: Option<ResMut<InitialFetchTask>>,
    game_state: Option<ResMut<GameState>>,
) {
    if let (Some(mut task_res), Some(mut game_state)) = (task_res, game_state) {
        if let Some(result) = bevy::tasks::block_on(bevy::tasks::poll_once(&mut task_res.0)) {
            match result {
                Ok(models) => {
                    info!("Initial entity fetch completed, updating game state...");
                    
                    // Update game state with all fetched models
                    for model in models {
                        game_state.update_from_model(&model);
                    }
                    
                    info!("Game state initialized with:");
                    info!("  - {} MoonRocks entries", game_state.moon_rocks.len());
                    info!("  - {} Games", game_state.games.len());
                    info!("  - {} GameCounters", game_state.game_counters.len());
                    info!("  - {} ActiveGames", game_state.active_games.len());
                    info!("  - {} OrbBagSlots", game_state.orb_bag_slots.len());
                    info!("  - {} DrawnOrbs", game_state.drawn_orbs.len());
                    info!("  - {} ShopInventory items", game_state.shop_inventory.len());
                    info!("  - {} PurchaseHistory entries", game_state.purchase_history.len());
                }
                Err(e) => {
                    error!("Failed to fetch initial entities: {}", e);
                }
            }
            commands.remove_resource::<InitialFetchTask>();
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
