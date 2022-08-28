use crate::states::playing::camera::GameCamera;
use crate::states::playing::floaty_text;
use crate::states::playing::floaty_text::{floaty_text_bundle, FloatyText, FONT};
use crate::states::playing::hover_stats::HoverStats;
use crate::states::playing::left_click::Selected;
use bevy::prelude::*;
use shared::game::defs::Defs;
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::SharedGame;
use shared::game::ClientGameInfo;

#[derive(Component)]
pub struct MouseHoverText;

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Query<&SharedGame>,
    defs: Res<Defs>,
    mut camera: Query<&mut GameCamera>,
    game_info: Query<&ClientGameInfo>,
) {
    commands.insert_resource(Selected::Nothing);
    commands
        .spawn_bundle(floaty_text::floaty_text_bundle(&asset_server))
        .insert(FloatyText::default())
        .insert(MouseHoverText);

    let game_info = game_info.single();
    let owner = game_info.i_am;
    // Find the player's home base and set the camera to it.
    let base = defs
        .levels
        .get(&game_info.map)
        .unwrap()
        .entities
        .iter()
        .find(|e| e.owner == Some(owner) && e.entity_type == shared::game::defs::EntityType::Base)
        .unwrap();

    let mut camera = camera.single_mut();
    camera.target = base.position.as_ref().unwrap().into();

    let mut text_bundle = floaty_text_bundle(&asset_server);
    text_bundle.style.position.right = Val::Px(16.0);
    text_bundle.style.position.bottom = Val::Px(16.0);
    text_bundle.style.align_content = AlignContent::FlexEnd;
    text_bundle.text = Text::from_sections(vec![
        TextSection::new(
            "Title\n",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 25.0,
                color: Color::BLACK,
            },
        ),
        TextSection::new(
            "Owner\n",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 20.0,
                color: Color::BLUE,
            },
        ),
        TextSection::new(
            "Info Text\nblah blah\nblahaaaaaaaaaaa",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 20.0,
                color: Color::DARK_GRAY,
            },
        ),
    ])
    .with_alignment(TextAlignment::BOTTOM_RIGHT);
    commands.spawn_bundle(text_bundle).insert(HoverStats);
}
