use crate::app::GameState;
use crate::net::GameOverEvent;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;
use iyes_loopless::prelude::NextState;
use shared::game::shared_game::SharedGame;

pub fn game_over(
    mut game_over_events: EventReader<GameOverEvent>,
    mut game: Query<&mut SharedGame>,
) {
    for game_over_event in game_over_events.iter() {
        let mut game = if let Ok(mut game) = game.get_single_mut() {
            game
        } else {
            warn!("No game when game over");
            return;
        };

        game.winner = Some(game_over_event.winner);
    }
}

pub fn game_over_message(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut game: Query<&mut SharedGame>,
) {
    let game = if let Ok(game) = game.get_single_mut() {
        game
    } else {
        return;
    };

    if let Some(winner) = game.winner {
        let winning_player = &game.players[winner.0 as usize];

        egui::Window::new("Game Over")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("Game Over! {} wins!", winning_player.name));
                ui.separator();
                if ui.button("Main Menu").clicked() {
                    commands.insert_resource(NextState(GameState::MainMenu));
                }
            });
    }
}
