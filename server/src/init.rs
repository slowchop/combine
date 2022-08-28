use crate::state::{GameUserLookup, PlayerQueue};
use bevy_ecs::prelude::*;
use bevy_ecs::system::Commands;
use bevy_log::info;
use naia_bevy_server::{Server, ServerAddrs};
use shared::game::defs::Defs;
use shared::protocol::Protocol;
use shared::{Channels, PUBLIC_WEBRTC_PORT, SESSION_LISTEN_PORT, URL, WEBRTC_LISTEN_ADDR};
use std::collections::HashMap;

pub fn init(mut server: Server<Protocol, Channels>) {
    info!("Server running! {}", URL);

    let public_webrtc_url = format!("{}:{}", URL, PUBLIC_WEBRTC_PORT);
    let server_addresses = ServerAddrs::new(
        format!("0.0.0.0:{}", SESSION_LISTEN_PORT)
            .parse()
            .expect("could not parse Signaling address/port"),
        // IP Address to listen on for UDP WebRTC data channels
        format!("0.0.0.0:{}", WEBRTC_LISTEN_ADDR)
            .parse()
            .expect("could not parse WebRTC data address/port"),
        // The public WebRTC IP address to advertise
        &public_webrtc_url,
    );

    server.listen(&server_addresses);
}
