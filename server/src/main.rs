mod cold;
mod create_games;
mod creeps;
mod damage;
mod destroy;
mod events;
mod game_over;
mod init;
mod match_randoms;
mod new_entities;
mod release_creeps;
mod spawn_entities;
mod state;
mod stats;
mod tick;
mod time;
mod towers;

use crate::cold::monitor_cold_changes;
use crate::create_games::create_games;
use crate::create_games::CreateGameEvent;
use crate::creeps::{move_along_path, spawn_creeps, CreepNeedsPositionUpdate};
use crate::damage::damage_creeps;
use crate::destroy::{destroy_entities, DestroyEntityEvent};
use crate::game_over::game_over;
use crate::new_entities::{add_new_entities_to_game, NewEntityEvent};
use crate::release_creeps::tell_clients_to_release_the_creeps;
use crate::spawn_entities::{spawn_entities, SpawnEntityEvent};
use crate::state::{GameLookup, GamePlayerHasDisconnected, GameUserLookup, PlayerQueue};
use crate::stats::{lose_a_life, GameOverEvent, LoseALifeEvent};
use crate::time::{add_ticks_to_games, emit_time_events, ReleaseCreepsEvent, SpawnCreepsEvent};
use crate::towers::{shoot_towers, DamageCreepEvent};
use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_ecs::prelude::*;
use bevy_log::{info, LogPlugin};
use bevy_time::TimePlugin;
use clap::Parser;
use init::init;
use match_randoms::match_randoms;
use naia_bevy_server::shared::ConnectionConfig;
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};
use shared::game::defs::Defs;
use shared::protocol::Protocol;
use shared::{shared_config, Channels};
use std::time::Duration;
use tick::tick;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {}

fn main() {
    info!("Server starting...");
    let _guard = sentry::init((
        "https://db16146d2a8743db88796811d9cb150c@o1376616.ingest.sentry.io/6686130",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let args = Args::parse();

    let server_config = ServerConfig {
        connection: ConnectionConfig {
            disconnection_timeout_duration: Duration::from_secs(5),
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
        .insert_resource(GameLookup::default())
        .insert_resource(GameUserLookup::default())
        .insert_resource(GamePlayerHasDisconnected::default())
        .add_event::<SpawnEntityEvent>()
        .add_event::<CreateGameEvent>()
        .add_event::<NewEntityEvent>()
        .add_event::<ReleaseCreepsEvent>()
        .add_event::<SpawnCreepsEvent>()
        .add_event::<DestroyEntityEvent>()
        .add_event::<LoseALifeEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<DamageCreepEvent>()
        .add_event::<CreepNeedsPositionUpdate>()
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
        .add_system(tell_clients_to_release_the_creeps)
        .add_system(spawn_creeps)
        .add_system(move_along_path)
        .add_system(destroy_entities)
        .add_system(lose_a_life)
        .add_system(game_over)
        .add_system(shoot_towers)
        .add_system(damage_creeps)
        .add_system(monitor_cold_changes)
        .run();
}
