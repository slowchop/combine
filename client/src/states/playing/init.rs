use crate::states::playing::floaty_text::FloatyText;
use crate::states::playing::left_click::Selected;
use bevy::prelude::*;

#[derive(Component)]
pub struct HoverText;

pub fn floaty_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/oliver/Oliver-Regular.ttf"),
        font_size: 40.0,
        color: Color::BLACK,
    }
}

pub fn floaty_style() -> Style {
    Style {
        max_size: Size {
            width: Val::Px(400.),
            height: Val::Undefined,
        },
        ..default()
    }
}

pub fn floaty_text_bundle(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_section("", floaty_text_style(&asset_server)).with_style(floaty_style())
}

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Selected::Nothing);
    commands
        .spawn_bundle(floaty_text_bundle(&asset_server))
        .insert(FloatyText::default())
        .insert(HoverText);
}
