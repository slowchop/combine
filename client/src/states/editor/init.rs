use bevy::prelude::ResMut;
use bevy_egui::egui::{FontFamily, FontId};
use bevy_egui::{egui, EguiContext};

pub fn init(mut egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx_mut();
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(20.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(20.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(20.0, FontFamily::Proportional),
    );
    ctx.set_style(style);
}
