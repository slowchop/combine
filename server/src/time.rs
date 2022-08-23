use crate::state::GameId;
use crate::GameLookup;
use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_time::Time;
use naia_bevy_server::Server;
use shared::protocol::Protocol;
use shared::{Channels, TICKS_PER_SECOND};
use std::time::Duration;

/// 0 is just after sunrise, so everything is bright.
/// 10 is when the sun starts setting.
/// 15 it gets dark -- Creeps released!
///
/// 28 "sunrise"
const SECONDS_PER_DAY: u64 = 30 * TICKS_PER_SECOND;
const RELEASE_CLOCK_TIME: u64 = 15 * TICKS_PER_SECOND;
const RESPAWN_CLOCK_TIME: u64 = 20 * TICKS_PER_SECOND;

pub struct ReleaseCreepsEvent(pub GameId);
pub struct RespawnCreepsEvent(pub GameId);

pub fn add_ticks_to_games(mut game_lookup: ResMut<GameLookup>) {
    for (game_id, game) in game_lookup.0.iter_mut() {
        game.tick();
    }
}

pub fn emit_time_events(
    game_lookup: Res<GameLookup>,
    mut release_creeps_events: EventWriter<ReleaseCreepsEvent>,
    mut respawn_creeps_events: EventWriter<RespawnCreepsEvent>,
) {
    for (game_id, game) in game_lookup.0.iter() {
        let clock = game.ticks() % SECONDS_PER_DAY;
        if clock == RELEASE_CLOCK_TIME {
            release_creeps_events.send(ReleaseCreepsEvent(*game_id));
        }
        if clock == RESPAWN_CLOCK_TIME {
            respawn_creeps_events.send(RespawnCreepsEvent(*game_id));
        }
    }
}
