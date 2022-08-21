use crate::app::GameState;
use crate::states::playing::{GameInfo, Player};
use crate::states::ContinueState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use shared::game_info::Owner;
use shared::player_name::PlayerName;

pub fn init(mut commands: Commands, time: Res<Time>) {
    println!("Main menu...");
}

// pub fn update(mut commands: Commands, time: Res<Time>) {
//     println!("Waiting for mainmenu...");
//     if time.seconds_since_startup() > 2.5 {
//         commands.insert_resource(NextState(GameState::Playing));
//     }
// }

pub fn update(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Combo Towers").show(egui_context.ctx_mut(), |ui| {
        if ui.button("AI").clicked() {
            println!("AI");
            commands.spawn().insert(GameInfo {
                level: "test".to_string(),
                players: [Player::human(PlayerName::random()), Player::ai()],
                you_are: Owner::new(0),
                multiplayer: false,
            });
            commands.insert_resource(NextState(GameState::LoadingLevel));
        };
        if ui.button("Multiplayer").clicked() {
            println!("Multiplayer");
            commands.insert_resource(ContinueState(Some(GameState::WaitingForRandom)));
            commands.insert_resource(NextState(GameState::Connecting));
        };
        if ui.button("Friend").clicked() {
            println!("Friend");
            commands.insert_resource(ContinueState(Some(GameState::VsFriend)));
            commands.insert_resource(NextState(GameState::Connecting));
        };
    });
}
