use crate::app::GameState;
use crate::states::ContinueState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::player_name::PlayerName;
use shared::protocol::Protocol;
use shared::{Auth, Channels, JoinRandomGame, UDP_PORT};

pub fn init(mut commands: Commands, time: Res<Time>, mut client: Client<Protocol, Channels>) {
    println!("Waiting for random...");
    let name = PlayerName::random();
    let command = JoinRandomGame::new(name);
    client.send_message(Channels::PlayerCommand, &command);
}

pub fn update(mut commands: Commands, time: Res<Time>, mut client: Client<Protocol, Channels>) {
    // if client.is_connected() {
    //     println!("Connected to: {}", client.server_address());
    // } else {
    //     println!("Waiting for connection...");
    // }
}
