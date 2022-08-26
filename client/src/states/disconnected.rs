use crate::app::GameState;
use crate::states::ContinueState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::egui::Align2;
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::game::player::PlayerName;
use shared::game::shared_game::SharedGame;
use shared::game::ClientGameInfo;
use shared::protocol::Protocol;
use shared::{Auth, Channels, JoinRandomGame};

pub fn init(
    mut commands: Commands,
    transforms: Query<Entity, (With<Transform>, Without<Camera>)>,
    game_infos: Query<Entity, With<ClientGameInfo>>,
    games: Query<Entity, With<SharedGame>>,
) {
    println!("Disconnected :(");

    for entity in transforms.iter() {
        commands.entity(entity).despawn();
    }
    for entity in game_infos.iter() {
        commands.entity(entity).despawn();
    }
    for entity in games.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn disconnected(mut egui_context: ResMut<EguiContext>, mut commands: Commands) {
    egui::Window::new("Disconnected :(")
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(
                "I'm very sorry but, either the server crashed or your Internet is a bit funky.",
            );
            ui.separator();
            ui.label("Please try again.");
            if ui.button("Back to main menu").clicked() {
                commands.insert_resource(NextState(GameState::MainMenu));
            }
        });
}
