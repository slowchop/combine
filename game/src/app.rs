use crate::camera::GameCamera;
use crate::net::{connect_event, disconnect_event, receive_message_event};
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
use naia_bevy_client::{Client, ClientConfig, Stage as NaiaStage};
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
        .add_system_to_stage(NaiaStage::Connection, connect_event)
        .add_system_to_stage(NaiaStage::Disconnection, disconnect_event)
        .add_system_to_stage(NaiaStage::ReceiveEvents, receive_message_event)
        .add_system_to_stage(NaiaStage::Tick, tick)
        .add_system_to_stage(NaiaStage::Frame, input)
        .add_system_to_stage(NaiaStage::PostFrame, sync)
        .add_startup_system(init)
        .add_system(quit_on_escape)
        .add_system(move_camera)
        .add_system(spawn_level)
        .run();
}

fn input() {}

fn sync() {}

fn tick(
    // mut global: ResMut<Global>,
    mut client: Client<Protocol, Channels>,
) {
    if let Some(client_tick) = client.client_tick() {
        println!("{:?}", client_tick);
        // if global.command_history.can_insert(&client_tick) {
        // Record command
        // global.command_history.insert(client_tick, command.clone());

        // Send command
        // client.send_message(Channels::PlayerCommand, &command);
        let command = Auth::new();
        client.send_message(Channels::PlayerCommand, &command);

        // Apply command
        // if let Ok(mut position) = position_query.get_mut(predicted_entity) {
        //     shared_behavior::process_command(&command, &mut position);
        // }
        // }
    }
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client: Client<Protocol, Channels>,
) {
    client.auth(Auth::new());
    client.connect(&format!("http://10.0.4.14:{}", UDP_PORT));
    // let command = Auth::new();
    // client.send_message(Channels::PlayerCommand, &command);

    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(GameCamera::default());

    commands.insert_resource(asset_server.load::<Textures, _>("game.textures"));
    commands.insert_resource(asset_server.load::<Level, _>("levels/test.level"));
}
