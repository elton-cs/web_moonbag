use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_background, spawn_moon).chain());
    app.init_resource::<BackgroundAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BackgroundAssets {
    #[dependency]
    starbg: Handle<Image>,
    #[dependency]
    moon: Handle<Scene>,
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
            moon: assets.load("models/moon.glb#Scene0"),
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
        Transform::from_translation(Vec3::new(0.0, 0.0, -2.0)) // Far behind in 3D space
            .with_scale(Vec3::splat(1.0)),
    ));
}

fn spawn_moon(mut commands: Commands, background_assets: Res<BackgroundAssets>) {
    commands.spawn((
        Name::new("Moon"),
        SceneRoot(background_assets.moon.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)) // Centered, in front of background
            .with_scale(Vec3::splat(1.0)),
    ));
}
