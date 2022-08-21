use crate::net::{connect_event, disconnect_event, receive_message_event};
use crate::other_systems::quit_on_escape;
use crate::settings::Settings;
use crate::states::playing::camera::GameCamera;
use crate::states::{connecting, loading, main_menu, playing, ContinueState};
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
use iyes_loopless::prelude::*;
use naia_bevy_client::Plugin as ClientPlugin;
use naia_bevy_client::{Client, ClientConfig, Stage as NaiaStage};
use shared::{shared_config, Auth, Channels, Protocol, UDP_PORT};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    PickName,
    MainMenu,
    Settings,

    Connecting,

    // Friends
    VsFriend,
    WaitForFriend,

    // Random friend
    WaitingForRandom,

    // In game!
    Playing,
}

pub fn play() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
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
    .insert_resource(ContinueState(None))
    .add_loopless_state(GameState::Loading)
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
    .add_system_to_stage(NaiaStage::PostFrame, sync);

    // Loading
    app.add_enter_system(GameState::Loading, loading::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Loading)
            .with_system(loading::update)
            .into(),
    );

    // Main Menu
    app.add_enter_system(GameState::MainMenu, main_menu::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::MainMenu)
            .with_system(main_menu::update)
            .into(),
    );

    // Connecting
    app.add_enter_system(GameState::Connecting, connecting::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Connecting)
            .with_system(connecting::update)
            .into(),
    );

    // Playing
    app.add_enter_system(GameState::Playing, init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Playing)
            .with_system(quit_on_escape)
            .with_system(move_camera)
            .with_system(spawn_level)
            .into(),
    );
    // .add_system(spawn_level)

    app.run()
}

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
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
    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(GameCamera::default());

    commands.insert_resource(asset_server.load::<Textures, _>("game.textures"));
    commands.insert_resource(asset_server.load::<Level, _>("levels/test.level"));
}
