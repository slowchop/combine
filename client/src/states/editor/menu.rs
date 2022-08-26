use crate::states::playing::console::ConsoleItem;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct NewEvent;

pub struct LoadEvent(String);

pub struct SaveEvent(String);

pub struct AddSprite(String);

pub fn menu(
    time: Res<Time>,
    mut egui_context: ResMut<EguiContext>,
    mut commands: Commands,
    // mut new_events: EventWriter<NewEvent>,
    mut console_events: EventWriter<ConsoleItem>,
) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        if ui.button("New").clicked() {
            console_events.send(ConsoleItem::new(format!(
                "New clicked {}",
                time.seconds_since_startup()
            )));
        };
    });
}
