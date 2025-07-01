use bevy::prelude::*;
use web_moonbag::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
