use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Startup,
        (spawn_background, spawn_moon, spawn_lighting).chain(),
    );
    app.add_systems(Update, rotate_moon);
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
        Mesh3d(meshes.add(Rectangle::new(20.0, 20.0))), // Double the original size
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
            .with_scale(Vec3::splat(0.5)), // 50% smaller
        Visibility::default(), // Ensure visibility in 3D scene
        MoonRotation,          // Add rotation component
    ));
}

fn spawn_lighting(mut commands: Commands) {
    // Add a directional light to illuminate the moon
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.8), // Warm white light
            illuminance: 10000.0,               // Bright enough to see the moon clearly
            shadows_enabled: false,             // Disable shadows for performance
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0_f32.to_radians(), // Angle down from above
            45.0_f32.to_radians(),  // Angle from the side
            0.0,
        )),
        Visibility::default(),
    ));

    // Add ambient light for softer overall illumination
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.4, 0.5), // Cool ambient light
        brightness: 500.0,                 // Subtle ambient lighting
        affects_lightmapped_meshes: true,
    });
}

/// Component to mark entities that should rotate like the moon
#[derive(Component)]
struct MoonRotation;

/// System to rotate the moon irregularly like a floating space rock
fn rotate_moon(time: Res<Time>, mut query: Query<&mut Transform, With<MoonRotation>>) {
    let elapsed = time.elapsed_secs();
    
    // Use sine and cosine functions with different frequencies to create irregular rotation
    let x_rotation_speed = 0.2 * (1.0 + 0.5 * (elapsed * 0.7).sin()); // Varies between 0.1 and 0.3
    let y_rotation_speed = 0.15 * (1.0 + 0.3 * (elapsed * 1.1).cos()); // Varies between ~0.105 and ~0.195
    let z_rotation_speed = 0.1 * (1.0 + 0.8 * (elapsed * 0.5).sin()); // Varies between 0.02 and 0.18

    let delta_x_rotation = x_rotation_speed * time.delta_secs();
    let delta_y_rotation = y_rotation_speed * time.delta_secs();
    let delta_z_rotation = z_rotation_speed * time.delta_secs();

    for mut transform in query.iter_mut() {
        // Rotate around all three axes with varying speeds for natural tumbling motion
        transform.rotate_x(delta_x_rotation);
        transform.rotate_y(delta_y_rotation);
        transform.rotate_z(delta_z_rotation);
    }
}
