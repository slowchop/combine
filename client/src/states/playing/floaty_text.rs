use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

#[derive(Component, Default)]
pub struct FloatyText {
    pub text: String,
    pub world_position: Vec3,
}

pub const FONT: &str = "fonts/oliver/Oliver-Regular.ttf";

pub fn floaty_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(FONT),
        font_size: 40.0,
        color: Color::BLACK,
    }
}

pub fn floaty_style() -> Style {
    Style {
        max_size: Size {
            width: Val::Undefined,
            height: Val::Undefined,
        },
        position_type: PositionType::Absolute,
        ..default()
    }
}

pub fn floaty_text_bundle(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section("", floaty_text_style(&asset_server)).with_style(floaty_style())
}

pub fn update_floaty_text_and_world_to_screen_pos(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut query: Query<(&FloatyText, &mut Style, &mut Text)>,
) {
    let (camera_transform, camera) = camera_query.single();

    for (floaty, mut style, mut text) in query.iter_mut() {
        let viewport_pos = match camera.world_to_viewport(camera_transform, floaty.world_position) {
            Some(pos) => pos,
            None => continue,
        };

        style.position.left = Val::Px(viewport_pos.x);
        style.position.bottom = Val::Px(viewport_pos.y);
        text.sections[0].value = floaty.text.clone();
    }
}
