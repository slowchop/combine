use crate::states::playing::console::ConsoleItem;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::egui::Align2;
use shared::game::defs::Defs;
use shared::game::owner::Owner;

#[derive(Default)]
pub struct EditorInfo {
    pub map_name: String,
    selected_sprite: String,
}

pub struct ClearEditorLevelEvent;

pub struct LoadEvent(pub String);

pub struct SaveEvent(pub String);

pub struct AddSpriteEvent(pub String);

pub struct AddPathEvent(pub Owner);

pub struct DeleteEvent(pub Entity);

pub struct MoveEvent(pub Entity, pub Vec3);

pub fn menu(
    defs: Res<Defs>,
    mut egui_context: ResMut<EguiContext>,
    mut new_events: EventWriter<ClearEditorLevelEvent>,
    mut load_events: EventWriter<LoadEvent>,
    mut save_events: EventWriter<SaveEvent>,
    mut add_sprite_events: EventWriter<AddSpriteEvent>,
    mut add_path_events: EventWriter<AddPathEvent>,
    mut editor_map: ResMut<EditorInfo>,
) {
    egui::Window::new("Editor")
        .default_pos(egui::Pos2::new(16., 16.))
        .default_width(260.)
        .show(egui_context.ctx_mut(), |ui| {
            if ui.button("New").clicked() {
                new_events.send(ClearEditorLevelEvent);
            };

            ui.separator();
            ui.label("Map name:");
            ui.text_edit_singleline(&mut editor_map.map_name);
            if ui.button("Load").clicked() {
                load_events.send(LoadEvent(editor_map.map_name.clone()));
            };
            if ui.button("Save").clicked() {
                save_events.send(SaveEvent(editor_map.map_name.clone()));
            };

            ui.separator();
            ui.heading("Sprites");
            egui::ComboBox::from_label("")
                .width(260.)
                .selected_text(&editor_map.selected_sprite)
                .show_ui(ui, |ui| {
                    let mut textures = defs.textures.iter().collect::<Vec<_>>();
                    textures.sort_by(|a, b| a.0.cmp(b.0));

                    for (name, def) in textures {
                        let s = format!("{} ({}x{})", name, def.size.x, def.size.y);
                        ui.selectable_value(&mut editor_map.selected_sprite, name.into(), s);
                    }
                });
            if ui.button("Add Sprite").clicked() {
                add_sprite_events.send(AddSpriteEvent(editor_map.selected_sprite.clone()));
            };

            ui.separator();
            ui.heading("A path waypoint");
            if ui.button("Player 1").clicked() {
                add_path_events.send(AddPathEvent(Owner::new(0)));
            };
            if ui.button("Player 2").clicked() {
                add_path_events.send(AddPathEvent(Owner::new(1)));
            };
        });
}