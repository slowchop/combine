use crate::app::GameState;
use crate::states::ContinueState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::protocol::Protocol;
use shared::{Auth, Channels, UDP_PORT};

pub fn init(mut commands: Commands, time: Res<Time>, mut client: Client<Protocol, Channels>) {
    println!("Connecting...");

    client.auth(Auth {});
    client.connect(&format!("http://10.0.4.14:{}", UDP_PORT));
    // let command = Auth::new();
    // client.send_message(Channels::PlayerCommand, &command);
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut client: Client<Protocol, Channels>,
    next_state: Res<ContinueState>,
) {
    let next_state = match next_state.0 {
        Some(state) => state,
        None => panic!("No next state in connecting update."),
    };

    if client.is_connected() {
        println!("Connected to: {}", client.server_address());
        commands.insert_resource(NextState(next_state));
    } else {
        println!("Waiting for connection...");
    }
}
