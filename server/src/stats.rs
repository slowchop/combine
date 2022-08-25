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

#[derive(Debug)]
pub struct GameOverEvent {
    pub game_id: GameId,
    pub winner: Owner,
}

pub fn lose_a_life(
    mut lose_a_life_events: EventReader<LoseALifeEvent>,
    mut game_lookup: ResMut<GameLookup>,
    mut server: Server<Protocol, Channels>,
    game_user_lookup: Res<GameUserLookup>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    for lose_a_life_event in lose_a_life_events.iter() {
        let game = if let Some(game) = game_lookup.0.get_mut(&lose_a_life_event.game_id) {
            game
        } else {
            warn!("No game when losing a life: {:?}", lose_a_life_event);
            continue;
        };
        if game.winner.is_some() {
            continue;
        }

        let player = if let Some(player) = game.get_player_mut(lose_a_life_event.who) {
            player
        } else {
            warn!("No player when losing a life: {:?}", lose_a_life_event);
            continue;
        };

        if player.lives == 0 {
            game_over_events.send(GameOverEvent {
                game_id: lose_a_life_event.game_id,
                winner: lose_a_life_event.who.other_player(),
            });
            continue;
        }

        player.lives -= 1;

        let message = UpdatePlayer::new(lose_a_life_event.who, player.gold, player.lives);
        send_message_to_game(
            &mut server,
            &lose_a_life_event.game_id,
            &*game_user_lookup,
            Channels::ServerCommand,
            &message,
        );
    }
}
