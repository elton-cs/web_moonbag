//! Resources for managing Torii client state.

use bevy::prelude::*;
use std::sync::Arc;
use torii_client::Client;
use torii_client::error::Error;

/// The current state of the Torii connection.
#[derive(Resource, Default, Debug)]
pub enum ToriiConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Failed(Error),
}

/// The active Torii client connection.
#[derive(Resource)]
pub struct ToriiClient {
    pub client: Arc<Client>,
}

/// Configuration for the Torii connection.
#[derive(Debug, Clone)]
pub struct ToriiConfig {
    pub url: String,
    pub world_address: String,
}

impl Default for ToriiConfig {
    fn default() -> Self {
        Self {
            url: "https://api.cartridge.gg/x/moonbagvibes/torii".to_string(),
            world_address: "0x04d9778a74d2c9e6e7e4a24cbe913998a80de217c66ee173a604d06dea5469c3"
                .to_string(),
        }
    }
}
