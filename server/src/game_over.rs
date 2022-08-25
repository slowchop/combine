use crate::release_creeps::send_message_to_game;
use crate::stats::GameOverEvent;
use crate::{GameLookup, GameUserLookup};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use naia_bevy_server::Server;
use shared::protocol::game_over::GameOver;
use shared::protocol::Protocol;
use shared::Channels;

pub fn game_over(
    mut game_over_events: EventReader<GameOverEvent>,
    mut game_lookup: ResMut<GameLookup>,
    mut server: Server<Protocol, Channels>,
    game_user_lookup: Res<GameUserLookup>,
) {
    for game_over_event in game_over_events.iter() {
        info!(?game_over_event);

        let game = if let Some(game) = game_lookup.0.get_mut(&game_over_event.game_id) {
            game
        } else {
            warn!("No game when game over: {:?}", game_over_event);
            continue;
        };

        game.winner = Some(game_over_event.winner);

        let game_over = GameOver::new(game_over_event.winner);
        send_message_to_game(
            &mut server,
            &game_over_event.game_id,
            &game_user_lookup,
            Channels::ServerCommand,
            &game_over,
        );
    }
}
