use crate::app::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

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
            commands.insert_resource(NextState(GameState::Playing));
        };
        if ui.button("Multiplayer").clicked() {
            println!("Multiplayer");
            commands.insert_resource(NextState(GameState::ConnectingRandom));
        };
        if ui.button("Friend").clicked() {
            println!("Friend");
            commands.insert_resource(NextState(GameState::ConnectingFriend));
        };
    });
}
