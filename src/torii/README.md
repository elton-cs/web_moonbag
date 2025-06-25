# Torii Plugin for Bevy

This module provides a Bevy plugin for integrating with Torii, the indexer for Dojo worlds.

## Features

- Async connection to Torii endpoints
- Bevy resource management for connection state
- Proper error handling and logging
- Extensible entity syncing system

## Architecture

- **`client.rs`** - Contains the async connection logic for Torii
- **`resources.rs`** - Defines Bevy resources for managing state
- **`systems.rs`** - Contains Bevy systems for connection management and entity syncing
- **`mod.rs`** - Plugin definition and public exports

## Connection Flow

1. The plugin initializes connection state as `Connecting`
2. An async task is spawned to connect to the Torii endpoint
3. Connection result is stored in thread-local storage (WASM compatibility)
4. The `check_connection_status` system polls for results and updates Bevy resources
5. Once connected, the `ToriiClient` resource becomes available
6. The `sync_entities` system can then be used to sync world state

## Usage

The plugin is automatically registered in `main.rs` and will attempt to connect to the configured Torii endpoint on startup. Systems can access the connection state and client through Bevy's resource system.

## Configuration

The default configuration connects to:
- URL: `https://api.cartridge.gg/x/moonbagvibes/torii`
- World Address: `0x04d9778a74d2c9e6e7e4a24cbe913998a80de217c66ee173a604d06dea5469c3`

This can be customized by modifying the `ToriiConfig::default()` implementation.