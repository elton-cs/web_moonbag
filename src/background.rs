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

fn spawn_background(
    mut commands: Commands, 
    background_assets: Res<BackgroundAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Background"),
        Mesh3d(meshes.add(Rectangle::new(10.0, 10.0))), // Large quad for background
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(background_assets.starbg.clone()),
            unlit: true, // No lighting for background
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, -5.0)) // Far behind in 3D space
            .with_rotation(Quat::from_rotation_x(0.0)), // Face the camera
        Visibility::default(),
    ));
}

fn spawn_moon(mut commands: Commands, background_assets: Res<BackgroundAssets>) {
    commands.spawn((
        Name::new("Moon"),
        SceneRoot(background_assets.moon.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)) // Centered, in front of background
            .with_scale(Vec3::splat(1.0)),
        Visibility::default(), // Ensure visibility in 3D scene
    ));
}
