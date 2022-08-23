use crate::state::GameId;
use crate::GameLookup;
use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_time::Time;
use naia_bevy_server::Server;
use shared::protocol::Protocol;
use shared::{Channels, RELEASE_CLOCK_TIME, RESPAWN_CLOCK_TIME, TICKS_PER_DAY, TICKS_PER_SECOND};
use std::time::Duration;

#[derive(Debug)]
pub struct ReleaseCreepsEvent {
    pub game_id: GameId,
}

#[derive(Debug)]
pub struct SpawnCreepsEvent {
    pub game_id: GameId,
}

pub fn add_ticks_to_games(mut game_lookup: ResMut<GameLookup>) {
    for game in game_lookup.0.values_mut() {
        game.tick();
    }
}

pub fn emit_time_events(
    game_lookup: Res<GameLookup>,
    mut release_creeps_events: EventWriter<ReleaseCreepsEvent>,
    mut respawn_creeps_events: EventWriter<SpawnCreepsEvent>,
) {
    for (game_id, game) in game_lookup.0.iter() {
        let clock = game.ticks_since_start_of_day();
        if clock == RELEASE_CLOCK_TIME {
            release_creeps_events.send(ReleaseCreepsEvent { game_id: *game_id });
        }
        if clock == RESPAWN_CLOCK_TIME {
            respawn_creeps_events.send(SpawnCreepsEvent { game_id: *game_id });
        }
    }
}
