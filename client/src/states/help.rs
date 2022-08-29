use crate::app::{GameState, ThisState};
use crate::settings::Settings;
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::floaty_text::FONT;
use crate::BillboardMaterial;
use bevy::asset::LoadState;
use bevy::prelude::*;
use glob::glob;
use iyes_loopless::prelude::*;
use shared::game::defs::Defs;
use std::f32::consts::TAU;
use std::path::Path;
use std::time::Duration;

pub struct PersistImages(pub Vec<Handle<Image>>);
pub struct PersistFonts(pub Vec<Handle<Font>>);

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    defs: Res<Defs>,
) {
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
    let texture_info = defs.textures.get("menus/help.png").unwrap();
    let texture_height = texture_info.size.y;
    let window_height = window.height();
    let scale = window_height / texture_height;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("menus/help.png"),
            transform: Transform::from_scale(Vec3::new(scale, scale, 1.0)),
            ..Default::default()
        })
        .insert(ThisState);
}

pub fn update(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if buttons.just_released(MouseButton::Left)
        || keyboard.any_just_released([KeyCode::Space, KeyCode::Return, KeyCode::Escape])
    {
        commands.insert_resource(NextState(GameState::EnterName));
    }
}
