use crate::app::{GameState, ThisState};
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
    mut images: ResMut<PersistImages>,
    mut fonts: ResMut<PersistFonts>,
) {
    println!("Loading...");
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
    let texture_info = defs.textures.get("menus/splash.png").unwrap();
    let texture_height = texture_info.size.y;
    let window_height = window.height();
    let scale = window_height / texture_height;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("menus/splash.png"),
            transform: Transform::from_scale(Vec3::new(scale, scale, 1.0)),
            ..Default::default()
        })
        .insert(ThisState);

    // Try to load menus first (No idea if this works?)
    glob("assets/menus/*.png").unwrap().for_each(|path| {
        let path = path.unwrap();
        let path = path.strip_prefix("assets").unwrap();
        let handle: Handle<Image> = asset_server.load(path);
        images.0.push(handle);
    });
    glob("assets/**/*.ttf").unwrap().for_each(|path| {
        let path = path.unwrap();
        let path = path.strip_prefix("assets").unwrap();
        let handle: Handle<Font> = asset_server.load(path);
        fonts.0.push(handle);
    });
    glob("assets/**/*.png").unwrap().for_each(|path| {
        let path = path.unwrap();
        let path = path.strip_prefix("assets").unwrap();
        let handle: Handle<Image> = asset_server.load(path);
        images.0.push(handle)
    });
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<PersistImages>,
) {
    if time.time_since_startup() > Duration::from_secs(4) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}
