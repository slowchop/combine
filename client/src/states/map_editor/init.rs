use crate::app::MyRaycastSet;
use crate::states::playing::camera::GameCamera;
use bevy::prelude::{Camera3dBundle, Commands, ResMut};
use bevy_egui::egui::{FontFamily, FontId};
use bevy_egui::{egui, EguiContext};
use bevy_mod_raycast::RayCastSource;

pub fn init(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(GameCamera::default())
        .insert(RayCastSource::<MyRaycastSet>::new());

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
