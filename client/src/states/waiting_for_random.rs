use crate::app::GameState;
use crate::states::ContinueState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::egui::Align2;
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::game::player::PlayerName;
use shared::protocol::Protocol;
use shared::{Auth, Channels, JoinRandomGame};

pub fn init(
    mut commands: Commands,
    time: Res<Time>,
    mut client: Client<Protocol, Channels>,
    player_name: Res<PlayerName>,
) {
    println!("Waiting for random...");
    let name = player_name.clone();
    let command = JoinRandomGame::new(name);
    client.send_message(Channels::PlayerCommand, &command);
}

pub fn update(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Waiting for random")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {});
}
