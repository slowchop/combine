use crate::release_creeps::send_message_to_game;
use crate::state::GameId;
use crate::{GameLookup, GameUserLookup};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use naia_bevy_server::Server;
use shared::game::owner::Owner;
use shared::protocol::update_player::UpdatePlayer;
use shared::protocol::Protocol;
use shared::Channels;

#[derive(Debug)]
pub struct LoseALifeEvent {
    pub game_id: GameId,
    pub who: Owner,
}

pub fn lose_a_life(
    mut lose_a_life_events: EventReader<LoseALifeEvent>,
    mut game_lookup: ResMut<GameLookup>,
    mut server: Server<Protocol, Channels>,
    game_user_lookup: Res<GameUserLookup>,
) {
    for lose_a_life_event in lose_a_life_events.iter() {
        dbg!(lose_a_life_event);

        let game = if let Some(game) = game_lookup.0.get_mut(&lose_a_life_event.game_id) {
            game
        } else {
            warn!("No game when losing a life: {:?}", lose_a_life_event);
            continue;
        };

        let player = if let Some(player) = game.get_player_mut(lose_a_life_event.who) {
            player
        } else {
            warn!("No player when losing a life: {:?}", lose_a_life_event);
            continue;
        };

        let message = UpdatePlayer::new(lose_a_life_event.who, player.gold, player.lives);
        send_message_to_game(
            &mut server,
            &lose_a_life_event.game_id,
            &*game_user_lookup,
            Channels::ServerUpdate,
            &message,
        );
    }
}
