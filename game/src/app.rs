use crate::camera::GameCamera;
use crate::other_systems::quit_on_escape;
use crate::settings::Settings;
use crate::textures::update_texture_sizes;
use crate::{
    move_camera, spawn_level, AmbientLight, App, AssetServer, AssetServerSettings,
    BillboardMaterial, Camera3dBundle, ClearColor, Color, Commands, DefaultPlugins, Level,
    LevelLoadState, MaterialPlugin, Msaa, Res, Textures, Transform, Vec3, WindowDescriptor,
};
use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use clap::Parser;
use naia_bevy_client::Plugin as ClientPlugin;
use naia_bevy_client::{Client, ClientConfig};
use shared::{shared_config, Auth, Channels, Protocol, UDP_PORT};

pub fn play() {
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
        .insert_resource(Settings::default())
        .insert_resource(ClearColor(Color::rgb(0.1, 0.3, 0.4)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::BISQUE,
            brightness: 0.2,
        })
        .insert_resource(LevelLoadState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(ClientPlugin::<Protocol, Channels>::new(
            ClientConfig::default(),
            shared_config(),
        ))
        .add_plugin(YamlAssetPlugin::<Textures>::new(&["textures"]))
        .add_plugin(YamlAssetPlugin::<Level>::new(&["level"]))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MaterialPlugin::<BillboardMaterial>::default())
        .add_startup_system(init)
        .add_system(quit_on_escape)
        .add_system(move_camera)
        .add_system(spawn_level)
        .run();
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client: Client<Protocol, Channels>,
) {
    client.auth(Auth::new());
    client.connect(&format!("http://127.0.0.1:{}", UDP_PORT));

    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(GameCamera::default());

    commands.insert_resource(asset_server.load::<Textures, _>("game.textures"));
    commands.insert_resource(asset_server.load::<Level, _>("levels/test.level"));
}
