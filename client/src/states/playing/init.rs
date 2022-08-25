use crate::states::playing::left_click::Selected;
use bevy::prelude::*;

#[derive(Component)]
pub struct HoverText;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Selected::Nothing);

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Build tower here",
                TextStyle {
                    font: asset_server.load("fonts/oliver/Oliver-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            )
            .with_text_alignment(TextAlignment::TOP_RIGHT)
            .with_style(Style {
                max_size: Size {
                    width: Val::Px(400.),
                    height: Val::Undefined,
                },
                ..default()
            }),
        )
        .insert(HoverText);
}
