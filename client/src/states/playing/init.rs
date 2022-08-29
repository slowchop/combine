use crate::app::MyRaycastSet;
use crate::states::playing::camera::GameCamera;
use crate::states::playing::console::ConsoleItem;
use crate::states::playing::floaty_text;
use crate::states::playing::floaty_text::{floaty_text_bundle, FloatyText, FONT};
use crate::states::playing::hover_stats::HoverStats;
use crate::states::playing::left_click::Selected;
use crate::states::playing::top_helper_text::TopHelperText;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_mod_raycast::RayCastSource;
use shared::game::defs::Defs;
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::SharedGame;
use shared::game::ClientGameInfo;

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    defs: Res<Defs>,
    game_info: Query<&ClientGameInfo>,
    mut console: EventWriter<ConsoleItem>,
    mut other_cameras: Query<Entity, With<Camera>>,
) {
    for other_camera in other_cameras.iter() {
        commands.entity(other_camera).despawn();
    }

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
    let camera_target: Vec2 = base.position.as_ref().unwrap().into();
    let mut game_camera = GameCamera::default();
    game_camera.target = camera_target;

    commands.spawn_bundle(Camera2dBundle {
        camera: Camera {
            priority: 1,
            ..Default::default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..Default::default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            ..Default::default()
        })
        .insert(game_camera)
        .insert(RayCastSource::<MyRaycastSet>::new());

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

    // Top hover text
    let mut text_bundle = floaty_text_bundle(&asset_server);
    text_bundle.style.position.top = Val::Px(100.0);
    // text_bundle.style.position.left = Val::Px(16.0);
    // text_bundle.style.position.right = Val::Px(16.0);
    text_bundle.style.position_type = PositionType::Absolute;
    // text_bundle.node.size = Vec2::new(1000.0, 100.0);
    text_bundle.style.align_self = AlignSelf::FlexStart;
    // text_bundle.style.justify_content = JustifyContent::Center;
    // text_bundle.style.align_items = AlignItems::Center;
    text_bundle.text = Text::from_section(
        "",
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

pub fn exit(mut commands: Commands, mut other_cameras: Query<Entity, With<Camera>>) {
    for other_camera in other_cameras.iter_mut() {
        commands.entity(other_camera).despawn();
    }

    commands.spawn_bundle(Camera2dBundle::default());
}
