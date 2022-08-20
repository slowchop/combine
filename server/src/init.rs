use crate::state::State;
use bevy_ecs::system::Commands;
use bevy_log::info;
use naia_bevy_server::{Server, ServerAddrs};
use shared::{Channels, Protocol};
use std::collections::HashMap;

pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>) {
    info!("Server running!");

    let server_addresses = ServerAddrs::new(
        "0.0.0.0:24191"
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        "0.0.0.0:24192"
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        "http://127.0.0.1:14192",
    );

    server.listen(&server_addresses);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();

    // Resources
    commands.insert_resource(State {
        main_room_key,
        // user_to_prediction_map: HashMap::new(),
        // player_last_command: HashMap::new(),
    })
}
