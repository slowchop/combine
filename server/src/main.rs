mod create_games;
mod events;
mod init;
mod match_randoms;
mod new_entities;
mod spawn_entities;
mod state;
mod tick;
mod time;

use crate::create_games::create_games;
use crate::create_games::CreateGameEvent;
use crate::new_entities::{add_new_entities_to_game, send_new_entities_to_players, NewEntityEvent};
use crate::spawn_entities::{spawn_entities, SpawnServerEntityEvent};
use crate::state::{GameLookup, GameUserLookup, PlayerLookup, PlayerQueue};
use crate::time::{add_ticks_to_games, emit_time_events};
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_ecs::prelude::*;
use bevy_log::{info, LogPlugin};
use bevy_time::TimePlugin;
use init::init;
use match_randoms::match_randoms;
use naia_bevy_server::shared::ConnectionConfig;
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};
use shared::game::defs::Defs;
use shared::protocol::Protocol;
use shared::{shared_config, Channels};
use std::time::Duration;
use tick::tick;

fn main() {
    info!("Server starting...");

    let server_config = ServerConfig {
        connection: ConnectionConfig {
            disconnection_timeout_duration: Duration::from_secs(10),
            ..Default::default()
        },
        require_auth: true,
    };

    App::default()
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            server_config,
            shared_config(),
        ))
        .insert_resource(Defs::load())
        .insert_resource(PlayerQueue::default())
        .insert_resource(PlayerLookup::default())
        .insert_resource(GameLookup::default())
        .insert_resource(GameUserLookup::default())
        .add_event::<SpawnServerEntityEvent>()
        .add_event::<CreateGameEvent>()
        .add_event::<NewEntityEvent>()
        .add_startup_system(init)
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        .add_system_to_stage(Stage::Tick, tick)
        .add_system_to_stage(Stage::Tick, add_ticks_to_games)
        .add_system_to_stage(Stage::Tick, emit_time_events)
        .add_system(match_randoms)
        .add_system(create_games)
        .add_system(spawn_entities)
        .add_system(add_new_entities_to_game)
        .add_system(send_new_entities_to_players)
        .run();
}
