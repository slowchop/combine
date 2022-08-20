mod level;

use bevy::app::AppExit;
use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use crate::level::Level;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            resizable: false,
            width: 1024f32,
            height: 768f32,
            title: "Combined Towers".to_string(),
            ..Default::default()
        })
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.3, 0.4)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::BISQUE,
            brightness: 0.2,
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(YamlAssetPlugin::<Level>::new(&["level.yaml"]))
        .add_startup_system(init)
        .add_system(quit_on_escape)
        .run();
}

fn quit_on_escape(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn init(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::default(),
        ..Default::default()
    });
}