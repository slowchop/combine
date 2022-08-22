use crate::state::{Global, PlayerQueue, Players};
use bevy_ecs::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_log::{error, info};
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{Server, UserKey};
use shared::game::managed_game::ManagedGame;
use shared::game::player_name::PlayerName;
use shared::protocol::game_ready::GameReady;
use shared::protocol::Protocol;
use shared::Channels;

pub fn match_randoms(
    mut commands: Commands,
    mut player_queue: ResMut<PlayerQueue>,
    mut server: Server<Protocol, Channels>,
    mut players: ResMut<Players>,
) {
    loop {
        let found_players = match player_queue.pair() {
            None => return,
            Some(p) => p,
        };

        let player_names: [PlayerName; 2] = found_players
            .iter()
            .map(|&u| players.0.get(&u).unwrap().name.clone())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let room = server.make_room();
        let room_key = room.key();
        println!("Created room #{}", room_key.to_u64());

        println!("Creating managed game.");
        // commands.spawn().insert(ManagedGame::from_players_level_textures())

        for (idx, player) in found_players.iter().enumerate() {
            players.set_room(player, room_key);
            server.user_mut(&player).enter_room(&room_key);

            println!("Sending GameReady to {}", player.to_u64());
            let message = GameReady::new(player_names.clone(), idx as u8, "test".to_string());
            server.send_message(player, Channels::ServerCommand, &message);
        }

        info!("Joined room");
    }
}
