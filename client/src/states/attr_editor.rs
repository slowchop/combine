use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use shared::game::defs::{Defs, Tower};

pub fn attr_editor(mut egui_context: ResMut<EguiContext>, mut defs: ResMut<Defs>) {
    let mut towers = defs
        .towers
        .iter_mut()
        .map(|(k, v)| {
            debug_assert_eq!(k, &v.name);
            v
        })
        .collect::<Vec<&mut Tower>>();
    towers.sort_by_key(|t| t.name.0.clone());

    egui::Window::new("Towers").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.label("Emoji");
            ui.label("C"); // Cost $
            ui.label(" "); // Cost base towers
            ui.label("DOT"); // Damage over time
            ui.label("$/DOT"); // Cost per damage over time
            ui.label("Title");
            ui.label("Cost");
            ui.label("Damage");
            ui.label("Reload");
            ui.label("Range");
            ui.label("Reload");
            ui.label("Size");
            ui.end_row();

            for tower in towers {
                ui.label(tower.emoji.to_string());
            }

            ui.label("😬");
            ui.label("😬😬");
            ui.end_row();

            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.label("");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Same");
                ui.label("cell");
            });
            ui.label("?");
            ui.end_row();
        });
    });
}
