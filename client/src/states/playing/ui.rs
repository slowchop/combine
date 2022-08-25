use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use naia_bevy_client::Client;
use shared::game::shared_game::SharedGame;
use shared::protocol::Protocol;
use shared::ticks::Ticks;
use shared::Channels;

pub fn ui(
    mut egui_context: ResMut<EguiContext>,
    game: Query<&SharedGame>,
    client: Client<Protocol, Channels>,
) {
    let game = game.single();

    egui::Window::new("Players").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            for player in &game.players {
                ui.vertical(|ui| {
                    ui.label(player.name.to_string());
                    ui.label(format!("Gold: {}", player.gold));
                    ui.label(format!("Lives: {}", player.lives));
                });
            }
        });
    });

    egui::Window::new("General Stats").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Ticks: {:?}", game.ticks()));
        ui.label(format!("Time: {:?}", game.duration()));

        let ticks_left = Ticks(game.next_release_ticks().0.saturating_sub(game.ticks().0));
        ui.label(format!("Ticks until release: {:?}", ticks_left));
        ui.label(format!(
            "Time until release: {:?}",
            ticks_left.to_duration()
        ));
        ui.label(format!("Start of day: {:?}", game.start_of_day()));
        ui.label(format!(
            "Time of day: {:?}",
            game.ticks_since_start_of_day()
        ));

        ui.label(format!("Connected: {:?}", client.is_connected()));
        ui.label(format!("RTT: {:?}", client.rtt()));
    });
}
