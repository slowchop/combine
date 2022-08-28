use crate::net::{DestroyEntityEvent, GameOverEvent, ReleaseCreepEvent, UpdatePositionEvent};
use crate::settings::Settings;
use crate::states::attr_editor::attr_editor;
use crate::states::map_editor::add_buildable::add_buildable_area;
use crate::states::map_editor::add_path::add_path;
use crate::states::map_editor::add_sprite::add_sprite;
use crate::states::map_editor::input_events::{input_events, EditorDragState};
use crate::states::map_editor::load_map::{create_editor_entities, load_map, CreateEditorEntity};
use crate::states::map_editor::new::new_events;
use crate::states::map_editor::no_pointer_capture::{
    is_pointer_captured_system, IsPointerCaptured,
};
use crate::states::map_editor::path_lines::editor_lines;
use crate::states::map_editor::save_map::save_map;
use crate::states::playing::camera::GameCamera;
use crate::states::playing::console;
use crate::states::playing::console::ConsoleItem;
use crate::states::playing::creeps::release_creeps;
use crate::states::playing::debug_lines::{debug_lines_path, debug_lines_tower};
use crate::states::playing::destroy_entities::destroy_entities;
use crate::states::playing::floaty_text::update_floaty_text_and_world_to_screen_pos;
use crate::states::playing::game_over::{game_over, game_over_message};
use crate::states::playing::health_bars::{add_health_bars, health_bars};
use crate::states::playing::hover_stats::hover_stats;
use crate::states::playing::hurt_entities::{hurt_entities, HurtEntityEvent};
use crate::states::playing::left_click::mouse_action;
use crate::states::playing::spawn_entities::{spawn_entities, SpawnEntityEvent};
use crate::states::playing::time::add_ticks_to_game;
use crate::states::playing::ui::ui;
use crate::states::playing::update_player::{update_player, UpdatePlayerEvent};
use crate::states::playing::update_positions::{
    update_positions_from_server, update_transform_from_velocity, update_transforms_from_positions,
};
use crate::states::{
    connecting, loading_level, main_menu, playing, splash, waiting_for_random, ContinueState,
};
use crate::states::{disconnected, map_editor};
use crate::{
    move_camera, net, App, Args, AssetServer, AssetServerSettings, BillboardMaterial,
    Camera3dBundle, ClearColor, Color, Commands, DefaultPlugins, MaterialPlugin, Msaa, Res,
    Transform, Vec3, WindowDescriptor,
};
use bevy::prelude::*;
use bevy::render::render_resource::SamplerDescriptor;
use bevy::render::texture::ImageSettings;
use bevy::window::{PresentMode, WindowMode};
use bevy_egui::egui::{FontFamily, FontId};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, RayCastMesh, RayCastMethod, RayCastSource,
    RaycastSystem,
};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use iyes_loopless::prelude::*;
use naia_bevy_client::Plugin as ClientPlugin;
use naia_bevy_client::{Client, ClientConfig, Stage as NaiaStage};
use shared::game::defs::Defs;
use shared::protocol::Protocol;
use shared::{shared_config, Auth, Channels};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Splash,
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
    LoadingLevel,
    Playing,

    //
    Disconnected,

    // Editor
    Editor,
}

pub fn play(args: &Args) {
    let mut app = App::new();

    let position = match args.window_position_shift {
        Some(0) => WindowPosition::At(Vec2::new(800.0, 200.0)),
        Some(1) => WindowPosition::At(Vec2::new(1900.0, 200.0)),
        _ => WindowPosition::Automatic,
    };

    let window_mode = match args.window_position_shift {
        Some(0) => WindowMode::Windowed,
        Some(1) => WindowMode::Windowed,
        _ => WindowMode::BorderlessFullscreen,
    };

    let mut settings = Settings::default();
    if args.skip_to_random_player {
        settings.start_multiplayer_immediately = true;
    }

    let image_settings = ImageSettings::default_linear();

    app.insert_resource(WindowDescriptor {
        resizable: true,
        width: 1024f32,
        height: 768f32,
        title: "Tower Combos".to_string(),
        present_mode: PresentMode::AutoNoVsync,
        mode: window_mode,
        position,
        ..Default::default()
    })
    .insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..Default::default()
    })
    .insert_resource(settings)
    .insert_resource(Defs::load())
    .insert_resource(ClearColor(Color::rgb(0.1, 0.3, 0.4)))
    .insert_resource(image_settings)
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    })
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(ContinueState(None))
    .add_loopless_state(GameState::Splash)
    .add_plugins(DefaultPlugins);

    app.add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
        );

    app.add_plugin(ClientPlugin::<Protocol, Channels>::new(
        ClientConfig::default(),
        shared_config(),
    ))
    .add_plugin(DebugLinesPlugin::default())
    .add_plugin(EguiPlugin)
    // .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(ShapePlugin);

    // Ours!
    app.add_event::<SpawnEntityEvent>()
        .add_event::<ReleaseCreepEvent>()
        .add_event::<UpdatePositionEvent>()
        .add_event::<DestroyEntityEvent>()
        .add_event::<UpdatePlayerEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<ConsoleItem>()
        .add_event::<HurtEntityEvent>()
        .insert_resource(map_editor::menu::EditorInfo::default())
        .add_plugin(MaterialPlugin::<BillboardMaterial>::default())
        .add_system_to_stage(NaiaStage::Connection, net::connect_event)
        .add_system_to_stage(NaiaStage::Disconnection, net::disconnect_event)
        .add_system_to_stage(NaiaStage::ReceiveEvents, net::receive_message_event)
        .add_system_to_stage(NaiaStage::Tick, tick)
        .add_system_to_stage(NaiaStage::Tick, add_ticks_to_game);

    app.add_startup_system(init);
    app.add_startup_system(init_egui);

    // Splash
    app.add_enter_system(GameState::Splash, splash::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Splash)
            .with_system(splash::update)
            .into(),
    );

    // Main Menu
    app.add_enter_system(GameState::MainMenu, main_menu::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::MainMenu)
            .with_system(main_menu::update)
            // .with_system(attr_editor)
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

    // Wait for random
    app.add_enter_system(GameState::WaitingForRandom, waiting_for_random::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::WaitingForRandom)
            .with_system(waiting_for_random::update)
            .into(),
    );

    // Loading level
    app.add_enter_system(GameState::LoadingLevel, loading_level::init);

    // Playing
    app.add_enter_system(GameState::Playing, playing::init::init);
    app.add_exit_system(GameState::Playing, disconnected::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Playing)
            .with_system(mouse_action)
            .with_system(move_camera)
            .with_system(spawn_entities)
            .with_system(release_creeps)
            .with_system(ui)
            .with_system(update_positions_from_server)
            .with_system(update_transforms_from_positions)
            .with_system(update_transform_from_velocity)
            .with_system(destroy_entities)
            .with_system(update_player)
            .with_system(game_over)
            .with_system(game_over_message)
            .with_system(update_floaty_text_and_world_to_screen_pos)
            .with_system(debug_lines_path)
            .with_system(debug_lines_tower)
            .with_system(hurt_entities)
            .with_system(add_health_bars)
            .with_system(health_bars)
            .with_system(hover_stats)
            .with_system(console::handle_console_events)
            .with_system(console::update_console)
            .into(),
    );

    // Disconnected screen
    app.add_enter_system(GameState::Disconnected, disconnected::init);
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Disconnected)
            .with_system(disconnected::disconnected)
            .into(),
    );

    // Editor
    app.add_enter_system(GameState::Editor, map_editor::init::init);
    app.add_exit_system(GameState::Editor, init_egui);
    app.add_event::<CreateEditorEntity>()
        .insert_resource(EditorDragState::default())
        .insert_resource(IsPointerCaptured(false))
        .add_event::<map_editor::menu::ClearEditorLevelEvent>()
        .add_event::<map_editor::menu::SaveEditorLevelEvent>()
        .add_event::<map_editor::menu::LoadEditorLevelEvent>()
        .add_event::<map_editor::menu::AddEditorSpriteEvent>()
        .add_event::<map_editor::menu::AddEditorPathEvent>()
        .add_event::<map_editor::menu::AddEditorBuildableEvent>()
        .add_event::<map_editor::menu::DeleteEditorEntityEvent>()
        .add_event::<map_editor::menu::MoveEditorEntityEvent>();
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Editor)
            .with_system(map_editor::menu::menu)
            .with_system(console::handle_console_events)
            .with_system(console::update_console)
            .with_system(load_map)
            .with_system(create_editor_entities)
            .with_system(new_events)
            .with_system(move_camera)
            .with_system(editor_lines)
            .with_system(add_sprite)
            .with_system(input_events)
            .with_system(add_path)
            .with_system(save_map)
            .with_system(add_buildable_area)
            .with_system(is_pointer_captured_system)
            .into(),
    );

    app.run()
}

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// This is a unit struct we will use to mark our generic `RayCastMesh`s and `RayCastSource` as part
/// of the same group, or "RayCastSet". For more complex use cases, you might use this to associate
/// some meshes with one ray casting source, and other meshes with a different ray casting source."
pub struct MyRaycastSet;

// Update our `RayCastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<MyRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
}

// fn check_for_exit(
//     mut app_exit_events: EventReader<AppExit>,
//     mut client: Client<Protocol, Channels>,
// ) {
//     for event in app_exit_events.iter() {
//         println!("App exit event: {:?}", event);
//         client.disconnect();
//     }
// }

fn tick(
    // mut global: ResMut<Global>,
    mut client: Client<Protocol, Channels>,
) {
    if let Some(client_tick) = client.client_tick() {
        // println!("{:?}", client_tick);
        // if global.command_history.can_insert(&client_tick) {
        // Record command
        // global.command_history.insert(client_tick, command.clone());

        // Send command
        // client.send_message(Channels::PlayerCommand, &command);
        // let command = Auth::new();
        // client.send_message(Channels::PlayerCommand, &command);

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
        .insert(GameCamera::default())
        .insert(RayCastSource::<MyRaycastSet>::new());

    commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
}

fn init_egui(mut egui_context: ResMut<EguiContext>) {
    return;
    let ctx = egui_context.ctx_mut();
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(30.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(40.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(30.0, FontFamily::Proportional),
    );
    ctx.set_style(style);
}
