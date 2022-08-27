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

pub struct ClearEditorLevelEvent(pub String);

pub struct LoadEditorLevelEvent(pub String);

pub struct SaveEditorLevelEvent(pub String);

pub struct AddEditorSpriteEvent(pub String);

pub struct AddEditorPathEvent(pub Owner);

pub struct DeleteEditorEntityEvent(pub Entity);

pub struct MoveEditorEntityEvent(pub Entity, pub Vec3);

pub fn menu(
    defs: Res<Defs>,
    mut egui_context: ResMut<EguiContext>,
    mut new_events: EventWriter<ClearEditorLevelEvent>,
    mut load_events: EventWriter<LoadEditorLevelEvent>,
    mut save_events: EventWriter<SaveEditorLevelEvent>,
    mut add_sprite_events: EventWriter<AddEditorSpriteEvent>,
    mut add_path_events: EventWriter<AddEditorPathEvent>,
    mut editor_map: ResMut<EditorInfo>,
) {
    egui::Window::new("Editor")
        .default_pos(egui::Pos2::new(16., 16.))
        .default_width(260.)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("Map name:");
            ui.text_edit_singleline(&mut editor_map.map_name);

            ui.label("Pick a name before using New.");
            if ui.button("New").clicked() {
                new_events.send(ClearEditorLevelEvent(editor_map.map_name.clone()));
            };

            ui.label("Type a name before using Load to load that map.");
            if ui.button("Load").clicked() {
                load_events.send(LoadEditorLevelEvent(editor_map.map_name.clone()));
            };
            ui.label("Type a name before using Save, or it will overwrite the map.");
            if ui.button("Save").clicked() {
                save_events.send(SaveEditorLevelEvent(editor_map.map_name.clone()));
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
                add_sprite_events.send(AddEditorSpriteEvent(editor_map.selected_sprite.clone()));
            };

            ui.separator();
            ui.heading("A path waypoint");
            if ui.button("Player 1").clicked() {
                add_path_events.send(AddEditorPathEvent(Owner::new(0)));
            };
            if ui.button("Player 2").clicked() {
                add_path_events.send(AddEditorPathEvent(Owner::new(1)));
            };
        });
}
