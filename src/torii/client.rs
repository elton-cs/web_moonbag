use crate::torii::ToriiConfig;
use bevy::prelude::*;
use starknet::core::types::Felt;
use torii_client::Client;

pub async fn create_client(config: ToriiConfig) -> Result<Client, String> {
    let world_felt = Felt::from_hex_unchecked(&config.world_address);

    match Client::new(config.url, world_felt).await {
        Ok(client) => {
            let metadata = client
                .metadata()
                .await
                .map_err(|e| format!("Failed to get metadata: {}", e))?;

            info!(
                "Connected to Torii world at address: {}",
                metadata.world_address
            );
            Ok(client)
        }
        Err(e) => {
            error!("Failed to create Torii client: {}", e);
            Err(format!("Failed to create Torii client: {}", e))
        }
    }
}
