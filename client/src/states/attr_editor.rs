use bevy::prelude::*;
use bevy_egui::egui::{Label, TextEdit, Widget};
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
                ui.label("C");
                ui.label("C");
                ui.label("DOT");
                ui.label("$/DOT");

                TextEdit::singleline(&mut tower.title)
                    .desired_width(300.)
                    .show(ui);

                let mut s = format!("{}", tower.cost);
                if ui.text_edit_singleline(&mut s).changed() {
                    if let Ok(c) = s.parse() {
                        tower.cost = c;
                        defs.save();
                    }
                }

                let mut s = format!("{}", tower.damage);
                if ui.text_edit_singleline(&mut s).changed() {
                    if let Ok(c) = s.parse() {
                        tower.damage = c;
                        defs.save();
                    }
                }

                ui.end_row();
            }
        });
    });
}
