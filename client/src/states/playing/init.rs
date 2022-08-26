use crate::states::playing::floaty_text;
use crate::states::playing::floaty_text::FloatyText;
use crate::states::playing::left_click::Selected;
use bevy::prelude::*;

#[derive(Component)]
pub struct MouseHoverText;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Selected::Nothing);
    commands
        .spawn_bundle(floaty_text::floaty_text_bundle(&asset_server))
        .insert(FloatyText::default())
        .insert(MouseHoverText);
}
