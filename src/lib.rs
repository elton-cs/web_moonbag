use bevy::{asset::AssetMetaCheck, prelude::*};
mod torii;
mod display;
mod display_simple;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Web Moonbag".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );
        app.add_plugins(torii::plugin);
        app.add_plugins(display::DojoDisplayPlugin);
        // app.add_plugins(display_simple::DojoDisplayPlugin); // Replaced by full display module

        app.add_systems(Startup, spawn_camera);
    }
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
