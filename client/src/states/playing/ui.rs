use crate::states::playing::floaty_text::{floaty_text_bundle, FONT};
use bevy::prelude::*;
use bevy::text::Text2dSize;
use bevy_egui::{egui, EguiContext};
use naia_bevy_client::Client;
use shared::game::defs::Defs;
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::shared_game::{SharedGame, TimeLeft};
use shared::protocol::Protocol;
use shared::ticks::Ticks;
use shared::Channels;
use std::time::Duration;

#[derive(Component)]
pub struct TopStatusBackground;

#[derive(Component)]
pub struct TopIcons;

#[derive(Component)]
pub struct TopTimerNumber;

#[derive(Component)]
pub struct TopTimerText;

#[derive(Component)]
pub struct TopPlayerName;

#[derive(Component)]
pub struct TopGold;

#[derive(Component)]
pub struct TopLives;

const TOP_BACKGROUND_GRADIENT: &str = "ui/top-background-gradient.png";
const TOP_ICONS: &str = "ui/top-icons.png";

pub fn top_status_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Query<&SharedGame>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(TOP_BACKGROUND_GRADIENT),
            ..Default::default()
        })
        .insert(TopStatusBackground);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(TOP_ICONS),
            ..Default::default()
        })
        .insert(TopIcons);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_sections([TextSection::new(
                "29",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 70.0,
                    color: Color::BLACK,
                },
            )])
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(TopTimerNumber);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_sections([TextSection::new(
                "release",
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            )])
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(TopTimerText);

    for o in 0..2 {
        let owner = Owner::new(o);
        let alignment = match owner.0 {
            0 => HorizontalAlign::Left,
            1 => HorizontalAlign::Right,
            _ => unreachable!(),
        };
        let player = game.get_single().unwrap().get_player(owner).unwrap();
        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    player.name.0.clone(),
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 80.0,
                        color: Color::BLACK,
                    },
                )
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: alignment,
                }),
                ..Default::default()
            })
            .insert(TopPlayerName)
            .insert(owner);

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    "1021asdfdsafsa",
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 40.0,
                        color: Color::hex("479f5f").unwrap(),
                    },
                )
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: alignment,
                }),
                ..Default::default()
            })
            .insert(TopGold)
            .insert(owner);

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    "102112321321",
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 40.0,
                        color: Color::hex("e45b55").unwrap(),
                    },
                )
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Center,
                }),
                ..Default::default()
            })
            .insert(TopLives)
            .insert(owner);
    }
}

pub fn top_status_update(
    defs: Res<Defs>,
    windows: Res<Windows>,
    mut status_background: Query<&mut Transform, (With<TopStatusBackground>, Without<TopIcons>)>,
    mut icons: Query<&mut Transform, (With<TopIcons>, Without<TopStatusBackground>)>,
) {
    let window = windows.get_primary().unwrap();

    let mut transform = status_background.single_mut();
    let top_background_def = defs.textures.get(TOP_BACKGROUND_GRADIENT).unwrap();
    transform.translation.x = -window.width() / 2.0 - &top_background_def.size.x / 2.0;
    transform.translation.y = window.height() / 2.0 - &top_background_def.size.y / 2.0;
    transform.scale.x = window.width() / &top_background_def.size.y;

    let mut transform = icons.single_mut();
    let texture_def = defs.textures.get(TOP_ICONS).unwrap();
    transform.translation.y = window.height() / 2.0 - &texture_def.size.y / 2.0 - 20.;
    transform.translation.z = 1.0;
}

pub fn top_timer_number(
    windows: Res<Windows>,
    shared_game: Query<&SharedGame>,
    mut timer_number: Query<(&mut Transform, &mut Text, &Text2dSize), With<TopTimerNumber>>,
) {
    let window = windows.get_primary().unwrap();
    let game = shared_game.single();
    let time_left = game.time_left();
    let (mut transform, mut text, text_2d_size) = timer_number.single_mut();
    transform.translation.x = -text_2d_size.size.x / 2.0;
    transform.translation.y = window.height() / 2.0;
    transform.translation.z = 2.0;
    text.sections[0].value = format!("{:0.0}", time_left.duration().as_secs_f32());
}

pub fn top_timer_text(
    windows: Res<Windows>,
    shared_game: Query<&SharedGame>,
    mut timer_text: Query<(&mut Transform, &mut Text, &Text2dSize), With<TopTimerText>>,
) {
    let window = windows.get_primary().unwrap();
    let game = shared_game.single();
    let time_left = game.time_left();
    let (mut transform, mut text, text_2d_size) = timer_text.single_mut();
    transform.translation.x = -text_2d_size.size.x / 2.0 + 20.;
    transform.translation.y = window.height() / 2.0 - 60.;
    transform.translation.z = 2.0;
    text.sections[0].value = match time_left {
        TimeLeft::ReleaseCreeps(_) => "release".to_string(),
        TimeLeft::RespawnCreeps(_) => "respawn".to_string(),
    };
}

pub fn top_names(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &Owner), With<TopPlayerName>>,
) {
    let window = windows.get_primary().unwrap();
    for (mut transform, owner) in query.iter_mut() {
        let flip = if owner.0 == 1 { -1. } else { 1. };
        transform.translation.x = 666.0 * flip;
        transform.translation.y = window.height() / 2.0 - 10.;
        transform.translation.z = 2.0;
    }
}

pub fn top_gold(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Text, &Owner), With<TopGold>>,
    shared_game: Query<&SharedGame>,
) {
    let window = windows.get_primary().unwrap();
    let game = shared_game.single();
    for (mut transform, mut text, owner) in query.iter_mut() {
        let player = game.get_player(*owner).unwrap();
        text.sections[0].value = format!("{}", player.gold);

        let flip = if owner.0 == 1 { -1. } else { 1. };
        transform.translation.x = 260.0 * flip;
        transform.translation.y = window.height() / 2.0 - 30.;
        transform.translation.z = 2.0;
    }
}

pub fn top_lives(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Text, &Owner), With<TopLives>>,
    shared_game: Query<&SharedGame>,
) {
    let window = windows.get_primary().unwrap();
    let game = shared_game.single();
    for (mut transform, mut text, owner) in query.iter_mut() {
        let player = game.get_player(*owner).unwrap();
        text.sections[0].value = format!("{}", player.lives);

        let flip = if owner.0 == 1 { -1. } else { 1. };
        transform.translation.x = 520.0 * flip;
        transform.translation.y = window.height() / 2.0 - 30.;
        transform.translation.z = 2.0;
    }
}

pub fn ui_egui(
    mut egui_context: ResMut<EguiContext>,
    game: Query<&SharedGame>,
    client: Client<Protocol, Channels>,
) {
    return;
    let game = game.single();

    egui::Window::new("Players").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            for player in &game.players {
                ui.vertical(|ui| {
                    ui.label(player.name.to_string());
                    ui.label(format!("Gold: {}", player.gold));
                    ui.label(format!("Lives: {}", player.lives));
                });
            }
        });
    });

    egui::Window::new("General Stats").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Ticks: {:?}", game.ticks()));
        ui.label(format!("Time: {:?}", game.duration()));

        let ticks_left = Ticks(game.next_release_ticks().0.saturating_sub(game.ticks().0));
        ui.label(format!("Ticks until release: {:?}", ticks_left));
        ui.label(format!(
            "Time until release: {:?}",
            ticks_left.to_duration()
        ));
        ui.label(format!("Start of day: {:?}", game.start_of_day()));
        ui.label(format!(
            "Time of day: {:?}",
            game.ticks_since_start_of_day()
        ));

        ui.label(format!("Connected: {:?}", client.is_connected()));
        ui.label(format!("RTT: {:?}", client.rtt()));
    });
}
