use starknet::core::types::Felt;
use torii_client::Client;
use web_sys::console;

pub async fn connect_torii() {
    let torii_url = "https://api.cartridge.gg/x/moonbagvibes/torii";
    let world_address = "0x04d9778a74d2c9e6e7e4a24cbe913998a80de217c66ee173a604d06dea5469c3";
    let world_felt = Felt::from_hex_unchecked(world_address);

    let client = Client::new(torii_url.to_string(), world_felt)
        .await
        .unwrap();
    let word_addr = client.metadata().await.unwrap().world_address;

    console::log_1(&format!("World address: {}", word_addr).into());
}