use crate::states::playing::camera::GameCamera;
use crate::states::playing::floaty_text;
use crate::states::playing::floaty_text::FloatyText;
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
}
