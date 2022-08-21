use crate::state::{PlayerInfo, PlayerQueue, State};
use bevy_ecs::system::Commands;
use bevy_log::info;
use naia_bevy_server::{Server, ServerAddrs};
use shared::{Channels, Protocol, UDP_PORT, WEB_PORT};
use std::collections::HashMap;

pub fn init(mut commands: Commands, mut server: Server<Protocol, Channels>) {
    info!("Server running!");

    let public_webrtc_url = format!("http://127.0.0.1:{}", WEB_PORT);
    let server_addresses = ServerAddrs::new(
        format!("0.0.0.0:{}", UDP_PORT)
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        format!("0.0.0.0:{}", WEB_PORT)
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        &public_webrtc_url,
    );

    server.listen(&server_addresses);

    // Create a new, singular room, which will contain Users and Entities that they
    // can receive updates from
    let main_room_key = server.make_room().key();
    // dbg!(&main_room_key);

    // Resources
    commands.insert_resource(State { main_room_key });
    commands.insert_resource(PlayerQueue::default());
    commands.insert_resource(PlayerInfo::default());
}
