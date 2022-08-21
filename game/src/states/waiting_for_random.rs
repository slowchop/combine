use crate::app::GameState;
use crate::states::ContinueState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::game::player_name::PlayerName;
use shared::protocol::Protocol;
use shared::{Auth, Channels, JoinRandomGame, UDP_PORT};

pub fn init(mut commands: Commands, time: Res<Time>, mut client: Client<Protocol, Channels>) {
    println!("Waiting for random...");
    let name = PlayerName::random();
    let command = JoinRandomGame::new(name);
    client.send_message(Channels::PlayerCommand, &command);
}

pub fn update(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Waiting for random").show(egui_context.ctx_mut(), |ui| {});
}
