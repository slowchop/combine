use crate::app::GameState;
use crate::states::ContinueState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::protocol::Protocol;
use shared::{Auth, Channels, UDP_PORT, UDP_URL, WEB_CONNECT_PORT, WEB_PORT, WEB_URL};

pub fn init(mut commands: Commands, time: Res<Time>, mut client: Client<Protocol, Channels>) {
    println!("Connecting...");

    client.auth(Auth {});

    #[cfg(target_arch = "wasm32")]
    client.connect(&format!("{}:{}", WEB_URL, WEB_CONNECT_PORT));

    #[cfg(not(target_arch = "wasm32"))]
    client.connect(&format!("{}:{}", UDP_URL, UDP_PORT));

    // let command = Auth::new();
    // client.send_message(Channels::PlayerCommand, &command);
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut client: Client<Protocol, Channels>,
    next_state: Res<ContinueState>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Connecting to server...").show(egui_context.ctx_mut(), |ui| {});

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
