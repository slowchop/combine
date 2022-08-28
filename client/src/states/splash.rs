use crate::app::GameState;
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::floaty_text::FONT;
use crate::BillboardMaterial;
use bevy::asset::LoadState;
use bevy::prelude::*;
use glob::glob;
use iyes_loopless::prelude::*;
use std::f32::consts::TAU;
use std::path::Path;

pub fn init(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    println!("Loading...");
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
    let window_height = window.height();

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("menus/logo.png"),
        transform: Transform::from_translation(Vec3::new(0.0, window_height / 4.0, 5.0)),
        ..Default::default()
    });

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            "Loading...",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 50.0,
                color: Color::BLACK,
            },
        )
        .with_alignment(TextAlignment {
            horizontal: HorizontalAlign::Center,
            vertical: VerticalAlign::Center,
        }),
        transform: Default::default(),
        ..Default::default()
    });

    glob("assets/menus/*.png").unwrap().for_each(|path| {
        let path = path.unwrap();
        // let path = Path::new(&path.unwrap());
        let a: Handle<Image> = asset_server.load(path);
    });
}

pub fn update(mut commands: Commands, time: Res<Time>) {
    println!("Waiting for loading...");
    // commands.insert_resource(NextState(GameState::MainMenu));
}
