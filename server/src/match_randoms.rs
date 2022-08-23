use crate::create_games::CreateGameEvent;
use crate::spawn_entities::SpawnEntityEvent;
use crate::state::{GameLookup, GameUserLookup, PlayerLookup, PlayerQueue};
use bevy_ecs::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_log::{error, info};
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{Server, UserKey};
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::game::shared_game::SharedGame;
use shared::protocol::game_ready::GameReady;
use shared::protocol::Protocol;
use shared::Channels;

pub fn match_randoms(
    mut player_queue: ResMut<PlayerQueue>,
    mut create_game_events: EventWriter<CreateGameEvent>,
) {
    loop {
        let user_keys = match player_queue.find(2) {
            None => return,
            Some(p) => p,
        };

        create_game_events.send(CreateGameEvent { user_keys });
        info!("Matched 2 players!");
    }
}
