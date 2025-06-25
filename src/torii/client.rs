use crate::torii::ToriiConfig;
use bevy::prelude::*;
use starknet::core::types::Felt;
use torii_client::Client;
use torii_client::error::Error;

pub async fn create_client(config: ToriiConfig) -> Result<Client, Error> {
    match Client::new(config.url, Felt::from_hex_unchecked(&config.world_address)).await {
        Ok(c) => {
            info!("Created Torii client.");
            Ok(c)
        }
        Err(e) => {
            error!("Failed to create Torii client: {}", e);
            Err(e)
        }
    }
}
