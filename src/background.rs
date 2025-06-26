use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_background);
    app.init_resource::<BackgroundAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BackgroundAssets {
    #[dependency]
    starbg: Handle<Image>,
}

impl FromWorld for BackgroundAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            starbg: assets.load_with_settings(
                "images/starbg.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}

fn spawn_background(mut commands: Commands, background_assets: Res<BackgroundAssets>) {
    commands.spawn((
        Name::new("Background"),
        Sprite {
            image: background_assets.starbg.clone(),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)), // Behind other sprites
    ));
}