mod client;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, startup_torii);
}

fn startup_torii() {
    wasm_bindgen_futures::spawn_local(client::connect_torii());
    info!("Torii client only runs on WASM target");
}
