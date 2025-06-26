use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_moon);
    app.init_resource::<MoonAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MoonAssets {
    #[dependency]
    moon_model: Handle<Scene>,
}

impl FromWorld for MoonAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            moon_model: assets.load("models/moon.glb#Scene0"),
        }
    }
}

fn spawn_moon(mut commands: Commands, moon_assets: Res<MoonAssets>) {
    commands.spawn((
        Name::new("Moon"),
        SceneRoot(moon_assets.moon_model.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::splat(0.5)), // Scale down the moon
    ));
}