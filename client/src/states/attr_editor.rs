use bevy::prelude::*;
use bevy_egui::egui::{Label, TextEdit, Widget};
use bevy_egui::{egui, EguiContext};
use shared::game::defs::{Defs, Tower};

pub fn attr_editor(mut egui_context: ResMut<EguiContext>, mut defs: ResMut<Defs>) {
    defs.save();
    let tower_costs = defs.tower_costs.as_ref().unwrap().clone();
    let mut towers = defs
        .towers
        .iter_mut()
        .map(|(k, v)| {
            debug_assert_eq!(k, &v.name);
            v
        })
        .collect::<Vec<&mut Tower>>();
    towers.sort_by_key(|t| tower_costs.get(&t.name));

    egui::Window::new("Towers").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.label("Emoji");
            ui.label("T$"); // Cost $
            ui.label("TC"); // Cost base towers
            ui.label("DOT"); // Cost per damage
            ui.label("D/$"); // Cost per damage
            ui.label("ID");
            ui.label("Title");
            ui.label("Cost");
            ui.label("Damage");
            ui.label("Reload");
            ui.label("Range");
            ui.label("Size");
            ui.end_row();

            for tower in towers.iter_mut() {
                let (cost, base_towers) = tower_costs.get(&tower.name).unwrap();
                ui.label(tower.emoji.to_string());
                ui.label(format!("{}", cost));
                ui.label(format!("{}", base_towers));
                let dot = tower.damage as f32 / tower.reload;
                ui.label(format!("{:.2}", dot));
                ui.label(format!("{:0.2}", dot / *cost as f32));

                ui.label(tower.name.0.to_string());
                ui.label(tower.title.to_string());

                let mut s = format!("{}", tower.cost);
                if ui.text_edit_singleline(&mut s).changed() {
                    if let Ok(c) = s.parse() {
                        tower.cost = c;
                    }
                }

                let mut s = format!("{}", tower.damage);
                if ui.text_edit_singleline(&mut s).changed() {
                    if let Ok(c) = s.parse() {
                        tower.damage = c;
                    }
                }

                let mut s = format!("{:0.1}", tower.reload);
                if ui.text_edit_singleline(&mut s).changed() {
                    if let Ok(c) = s.parse() {
                        tower.reload = c;
                    }
                }

                let mut s = format!("{}", tower.range);
                if ui.text_edit_singleline(&mut s).changed() {
                    if let Ok(c) = s.parse() {
                        tower.range = c;
                    }
                }

                let mut s = format!("{:0.1}", tower.size);
                if ui.text_edit_singleline(&mut s).changed() {
                    if let Ok(c) = s.parse() {
                        tower.size = c;
                    }
                }

                ui.end_row();
            }
        });
    });
}
