use crate::torii::ToriiConfig;
use bevy::prelude::*;
use starknet::core::types::Felt;
use torii_client::Client;
use torii_client::error::Error;

pub async fn create_client(config: ToriiConfig) -> Result<Client, Error> {
    let world_felt = Felt::from_hex_unchecked(&config.world_address);

    match Client::new(config.url, world_felt).await {
        Ok(client) => {
            let metadata = client.metadata().await?;

            info!(
                "Connected to Torii world at address: {}",
                metadata.world_address
            );
            Ok(client)
        }
        Err(e) => {
            error!("Failed to create Torii client: {}", e);
            Err(e)
        }
    }
}
