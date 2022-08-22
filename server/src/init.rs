use crate::state::{GameUserLookup, PlayerLookup, PlayerQueue};
use bevy_ecs::prelude::*;
use bevy_ecs::system::Commands;
use bevy_log::info;
use naia_bevy_server::{Server, ServerAddrs};
use shared::game::defs::Defs;
use shared::protocol::Protocol;
use shared::{Channels, UDP_PORT, WEB_PORT};
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
}
