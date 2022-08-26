use crate::app::GameState;
use crate::settings::Settings;
use crate::states::ContinueState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::game::shared_game::SharedGame;
use shared::protocol::Protocol;
use shared::Channels;

pub fn init(mut commands: Commands, time: Res<Time>) {
    println!("Main menu...");
}

// pub fn update(mut commands: Commands, time: Res<Time>) {
//     println!("Waiting for mainmenu...");
//     if time.seconds_since_startup() > 2.5 {
//         commands.insert_resource(NextState(GameState::Playing));
//     }
// }

pub fn update(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    settings: Res<Settings>,
    mut client: Client<Protocol, Channels>,
) {
    if client.is_connected() {
        client.disconnect();
    }

    egui::Window::new("Combo Towers").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Multiplayer").clicked() || settings.start_multiplayer_immediately {
            commands.insert_resource(ContinueState(Some(GameState::WaitingForRandom)));
            commands.insert_resource(NextState(GameState::Connecting));
        };
        // if ui.button("Friend").clicked() {
        //     println!("Friend");
        //     commands.insert_resource(ContinueState(Some(GameState::VsFriend)));
        //     commands.insert_resource(NextState(GameState::Connecting));
        // };
        if ui.button("Editor").clicked() {
            commands.insert_resource(NextState(GameState::Editor));
        };
    });
}
