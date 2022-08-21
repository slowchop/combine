use crate::state::{PlayerQueue, Players, State};
use bevy_ecs::prelude::Res;
use bevy_ecs::system::ResMut;
use bevy_log::{error, info};
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{Server, UserKey};
use shared::player_name::PlayerName;
use shared::protocol::game_ready::GameReady;
use shared::protocol::Protocol;
use shared::Channels;

pub fn match_randoms(
    mut player_queue: ResMut<PlayerQueue>,
    mut server: Server<Protocol, Channels>,
    player_info: Res<Players>,
) {
    loop {
        let players = match player_queue.pair() {
            None => return,
            Some(p) => p,
        };

        println!("????");

        let player_names: [PlayerName; 2] = players
            .iter()
            .map(|&u| player_info.0.get(&u).unwrap().name.clone())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let room = server.make_room();
        let room_key = room.key();
        println!("Creating room {}", room_key.to_u64());
        for (idx, player) in players.iter().enumerate() {
            server.user_mut(&player).enter_room(&room_key);

            println!("Sending GameReady to {}", player.to_u64());
            let message = GameReady::new(player_names.clone(), idx as u8, "test".to_string());
            server.send_message(player, Channels::ServerCommand, &message);
        }

        info!("Joined room");
    }
}
