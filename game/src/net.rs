use bevy::prelude::*;
use naia_bevy_client::events::MessageEvent;
use naia_bevy_client::{Client, CommandsExt};
use shared::{Channels, Protocol};

pub fn connect_event(client: Client<Protocol, Channels>) {
    println!("Client connected to: {}", client.server_address());
}

pub fn disconnect_event(client: Client<Protocol, Channels>) {
    println!("Client disconnected from: {}", client.server_address());
}

pub fn receive_message_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut local: Commands,
    // mut global: ResMut<Global>,
    client: Client<Protocol, Channels>,
) {
    // dbg!(client.is_connected());
    for event in event_reader.iter() {
        println!("event");
        // if let MessageEvent(Channels::EntityAssignment, Protocol::EntityAssignment(message)) = event
        // {
        //     println!("Client disconnected from: {}", client.server_address());
        //     let assign = *message.assign;
        //
        //     let entity = message.entity.get(&client).unwrap();
        // }
    }
}
