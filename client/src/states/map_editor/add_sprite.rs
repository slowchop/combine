use crate::states::map_editor::load_map::CreateEditorEntity;
use crate::states::map_editor::menu::{AddEditorSpriteEvent, ClearEditorLevelEvent};
use crate::states::playing::camera::GameCamera;
use bevy::prelude::*;
use shared::game::defs::{EntityDef, EntityType};
use shared::game::position::vec3_to_vec2;
use shared::game::shared_game::ServerEntityId;

pub fn add_sprite(
    mut create_event: EventReader<AddEditorSpriteEvent>,
    mut create_editor_entity: EventWriter<CreateEditorEntity>,
    game_camera: Query<&GameCamera>,
) {
    let game_camera = game_camera.single();
    for event in create_event.iter() {
        let entity_def = EntityDef {
            texture: Some(event.0.clone()),
            entity_type: EntityType::Sprite,
            position: Some(game_camera.target.into()),
            server_entity_id: Some(ServerEntityId::random()),
            ..Default::default()
        };
        create_editor_entity.send(CreateEditorEntity(entity_def));
    }
}
