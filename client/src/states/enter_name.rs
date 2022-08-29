use crate::app::{GameState, ThisState};
use crate::settings::Settings;
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::floaty_text::FONT;
use crate::states::splash::{PersistFonts, PersistImages};
use crate::BillboardMaterial;
use bevy::asset::LoadState;
use bevy::prelude::*;
use glob::glob;
use iyes_loopless::prelude::*;
use shared::game::defs::Defs;
use shared::game::player::PlayerName;
use std::f32::consts::TAU;
use std::path::Path;
use std::time::Duration;

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    defs: Res<Defs>,
    mut name: ResMut<PlayerName>,
) {
    println!("Enter name...");
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
    let texture_info = defs.textures.get("menus/name.png").unwrap();
    let texture_height = texture_info.size.y;
    let window_height = window.height();
    let scale = window_height / texture_height;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("menus/name.png"),
            transform: Transform::from_scale(Vec3::new(scale, scale, 1.0)),
            ..Default::default()
        })
        .insert(ThisState);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "".to_string(),
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 200.0 * scale,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            transform: Transform::from_translation(Vec3::new(0.0, window_height * 0.1, 5.0)),
            ..Default::default()
        })
        .insert(ThisState);
}

pub fn update(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut name: ResMut<PlayerName>,
    mut query: Query<&mut Text>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    settings: Res<Settings>,
) {
    if settings.start_multiplayer_immediately {
        *name = PlayerName::random();
        commands.insert_resource(NextState(GameState::MainMenu));
    }

    if keys.just_pressed(KeyCode::Return) {
        // Feed it back in so we can handle bad names
        *name = PlayerName::new(name.0.as_str());
        commands.insert_resource(NextState(GameState::MainMenu));
        return;
    }

    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
    let window_height = window.height();
    let cursor_y = match window.cursor_position() {
        Some(cursor_position) => cursor_position.y,
        None => return,
    };
    let vertical_fraction = cursor_y / window_height;

    // Play
    if buttons.just_pressed(MouseButton::Left) {
        if vertical_fraction < 0.25 {
            commands.insert_resource(NextState(GameState::MainMenu));
        }
    }

    for key in keys.get_just_pressed() {
        if key == &KeyCode::Back {
            name.0.pop();
            continue;
        }

        if name.0.len() == 3 {
            continue;
        }

        let repr = *key as u8;
        let add = if repr >= KeyCode::A as u8 && repr <= KeyCode::Z as u8 {
            (repr - KeyCode::A as u8 + 'A' as u8) as char
        } else {
            continue;
        };

        name.0.push(add);
    }

    query.single_mut().sections[0].value = name.0.clone();
}
