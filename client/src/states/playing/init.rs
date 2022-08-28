use crate::states::playing::camera::GameCamera;
use crate::states::playing::console::ConsoleItem;
use crate::states::playing::floaty_text;
use crate::states::playing::floaty_text::{floaty_text_bundle, FloatyText, FONT};
use crate::states::playing::hover_stats::HoverStats;
use crate::states::playing::left_click::Selected;
use crate::states::playing::top_helper_text::TopHelperText;
use bevy::prelude::*;
use shared::game::defs::Defs;
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::SharedGame;
use shared::game::ClientGameInfo;

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Query<&SharedGame>,
    defs: Res<Defs>,
    mut camera: Query<&mut GameCamera>,
    game_info: Query<&ClientGameInfo>,
    mut console: EventWriter<ConsoleItem>,
) {
    console.send(ConsoleItem::new(
        "Protect your home from the creeps baddies.".to_string(),
    ));
    console.send(ConsoleItem::new(
        "Combine your towers and creeps to upgrade them.".to_string(),
    ));
    console.send(ConsoleItem::new(
        "You have your own creeps that you can upgrade which are near your home.".to_string(),
    ));

    commands.insert_resource(Selected::Nothing);

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

    // Top hover text
    let mut text_bundle = floaty_text_bundle(&asset_server);
    text_bundle.style.position.top = Val::Px(100.0);
    text_bundle.style.position.left = Val::Px(16.0);
    text_bundle.style.position.right = Val::Px(16.0);
    text_bundle.style.position_type = PositionType::Absolute;
    text_bundle.style.align_self = AlignSelf::Center;
    text_bundle.style.justify_content = JustifyContent::Center;
    text_bundle.style.align_items = AlignItems::Center;
    text_bundle.text = Text::from_section(
        "testing 123./.........",
        TextStyle {
            font: asset_server.load(FONT),
            font_size: 40.0,
            color: Color::BLACK,
        },
    )
    .with_alignment(TextAlignment::TOP_CENTER);
    commands
        .spawn_bundle(text_bundle)
        .insert(TopHelperText("Top Helper Text".to_string()));

    // Bottom right hover text
    let mut text_bundle = floaty_text_bundle(&asset_server);
    text_bundle.style.position.right = Val::Px(16.0);
    text_bundle.style.position.bottom = Val::Px(16.0);
    text_bundle.text = Text::from_sections(vec![
        TextSection::new(
            "\n",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 30.0,
                color: Color::BLACK,
            },
        ),
        TextSection::new(
            "\n",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 25.0,
                color: Color::BLUE,
            },
        ),
        TextSection::new(
            "",
            TextStyle {
                font: asset_server.load(FONT),
                font_size: 25.0,
                color: Color::DARK_GRAY,
            },
        ),
    ])
    .with_alignment(TextAlignment::BOTTOM_RIGHT);
    commands.spawn_bundle(text_bundle).insert(HoverStats);
}
