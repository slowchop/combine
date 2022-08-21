mod events;
mod game_info;
mod init;
mod match_randoms;
mod state;
mod tick;

use bevy_app::{App, ScheduleRunnerPlugin};
use bevy_core::CorePlugin;
use bevy_ecs::prelude::*;
use bevy_log::{info, LogPlugin};
use bevy_time::TimePlugin;
use init::init;
use match_randoms::match_randoms;
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig, Stage};
use shared::protocol::Protocol;
use shared::{shared_config, Channels};
use tick::tick;

fn main() {
    info!("Server starting...");

    // Build App
    App::default()
        // Plugins
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(ServerPlugin::<Protocol, Channels>::new(
            ServerConfig::default(),
            shared_config(),
        ))
        // Startup System
        .add_startup_system(init)
        // Receive Server Events
        .add_system_to_stage(Stage::ReceiveEvents, events::authorization_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::connection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::disconnection_event)
        .add_system_to_stage(Stage::ReceiveEvents, events::receive_message_event)
        // Gameplay Loop on Tick
        .add_system_to_stage(Stage::Tick, tick)
        // Run App
        .add_system(match_randoms)
        .run();
}
