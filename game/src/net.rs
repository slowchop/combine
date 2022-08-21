use crate::app::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::NextState;
use naia_bevy_client::events::MessageEvent;
use naia_bevy_client::{Client, CommandsExt};
use shared::game_info::GameInfo;
use shared::protocol::Protocol;
use shared::Channels;

pub fn connect_event(client: Client<Protocol, Channels>) {
    println!("Client connected to: {}", client.server_address());
}

pub fn disconnect_event(client: Client<Protocol, Channels>) {
    println!("Client disconnected from: {}", client.server_address());
}

pub fn receive_message_event(
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut game_info: ResMut<GameInfo>,
    client: Client<Protocol, Channels>,
) {
    // dbg!(client.is_connected());
    for event in event_reader.iter() {
        println!("event");
        if let MessageEvent(Channels::ServerCommand, msg) = event {
            match msg {
                Protocol::Auth(_) => {}
                Protocol::JoinRandomGame(_) => {}
                Protocol::JoinFriendGame(_) => {}
                Protocol::GameReady(game_ready) => {
                    println!("Client got a game ready! {}", *game_ready.level);
                    let game_info: GameInfo = game_ready.into();
                    commands.insert_resource(game_info);
                    commands.insert_resource(NextState(GameState::LoadingLevel));
                }
            }
        }
    }
}
