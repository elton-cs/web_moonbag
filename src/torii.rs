use bevy::{prelude::*, tasks::IoTaskPool};
use starknet::core::types::Felt;
use torii_client::Client;

#[derive(Resource)]
pub struct ToriiClient {
    pub client: Client,
}

#[derive(Resource)]
struct ToriiClientTask(bevy::tasks::Task<Result<Client, Box<dyn std::error::Error + Send + Sync>>>);

pub struct ToriiPlugin;

impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_torii_client)
            .add_systems(Update, poll_torii_client_task);
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

fn poll_torii_client_task(
    mut commands: Commands,
    mut task_res: Option<ResMut<ToriiClientTask>>,
) {
    if let Some(mut task_res) = task_res {
        if let Some(result) = bevy::tasks::block_on(bevy::tasks::poll_once(&mut task_res.0)) {
            match result {
                Ok(client) => {
                    info!("Torii client created successfully");
                    commands.insert_resource(ToriiClient { client });
                }
                Err(e) => {
                    error!("Failed to create Torii client: {}", e);
                }
            }
            commands.remove_resource::<ToriiClientTask>();
        }
    }
}
