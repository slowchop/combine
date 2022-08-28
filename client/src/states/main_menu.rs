use crate::app::{GameState, ThisState};
use crate::settings::Settings;
use crate::states::playing::floaty_text::FONT;
use crate::states::ContinueState;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use naia_bevy_client::Client;
use shared::game::defs::Defs;
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::game::shared_game::SharedGame;
use shared::protocol::Protocol;
use shared::Channels;
use std::process::exit;

pub fn init(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    defs: Res<Defs>,
    mut windows: ResMut<Windows>,
    player_name: Res<PlayerName>,
) {
    println!("Main menu...");

    let texture_info = defs.textures.get("menus/main-menu.png").unwrap();
    let window = windows.get_primary_mut().unwrap();
    let texture_height = texture_info.size.y;
    let window_height = window.height();
    let scale = window_height / texture_height;
    // let scale = 1.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("menus/main-menu.png"),
            transform: Transform::from_scale(Vec3::new(scale, scale, 1.0)),
            ..Default::default()
        })
        .insert(ThisState);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                player_name.0.to_string(),
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

pub fn menu_clicks(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut client: Client<Protocol, Channels>,
    keys: Res<Input<KeyCode>>,
) {
    if client.is_connected() {
        client.disconnect();
    }

    if keys.just_pressed(KeyCode::Escape) {
        exit(0);
    }
    if keys.just_pressed(KeyCode::Return) {
        commands.insert_resource(NextState(GameState::WaitingForRandom));
        return;
    }

    if !buttons.just_pressed(MouseButton::Left) {
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
    if vertical_fraction > 0.345 && vertical_fraction < 0.49 {
        commands.insert_resource(ContinueState(Some(GameState::WaitingForRandom)));
        commands.insert_resource(NextState(GameState::Connecting));
    }

    // Exit
    if vertical_fraction > 0.09 && vertical_fraction < 0.22 {
        exit(0);
    }

    println!(
        "click at {:?} {}",
        window.cursor_position(),
        vertical_fraction
    );
}

pub fn egui(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    settings: Res<Settings>,
    mut client: Client<Protocol, Channels>,
) {
    if client.is_connected() {
        client.disconnect();
    }

    egui::Window::new("Combo Towers").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Multiplayer").clicked() || settings.start_multiplayer_immediately {
            commands.insert_resource(ContinueState(Some(GameState::WaitingForRandom)));
            commands.insert_resource(NextState(GameState::Connecting));
        };
        // if ui.button("Friend").clicked() {
        //     println!("Friend");
        //     commands.insert_resource(ContinueState(Some(GameState::VsFriend)));
        //     commands.insert_resource(NextState(GameState::Connecting));
        // };
        if ui.button("Editor").clicked() {
            commands.insert_resource(NextState(GameState::Editor));
        };
    });
}
