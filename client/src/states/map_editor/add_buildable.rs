use crate::states::map_editor::load_map::CreateEditorEntity;
use crate::states::map_editor::menu::{
    AddEditorBuildableEvent, AddEditorSpriteEvent, ClearEditorLevelEvent, EditorInfo,
};
use crate::states::playing::camera::GameCamera;
use bevy::prelude::*;
use shared::game::defs::{Defs, EntityDef, EntityType};
use shared::game::position::vec3_to_vec2;
use shared::game::shared_game::ServerEntityId;

pub fn add_buildable_area(
    mut defs: ResMut<Defs>,
    editor_info: Res<EditorInfo>,
    mut create_event: EventReader<AddEditorBuildableEvent>,
    mut create_editor_entity: EventWriter<CreateEditorEntity>,
    game_camera: Query<&GameCamera>,
) {
    let level = match defs.levels.get_mut(&editor_info.map_name) {
        None => {
            return;
        }
        Some(m) => m,
    };

    let game_camera = game_camera.single();
    for event in create_event.iter() {
        let entity_def = EntityDef {
            entity_type: EntityType::BuildableCircle,
            position: Some(game_camera.target.into()),
            radius: Some(event.0),
            owner: Some(event.1),
            server_entity_id: Some(ServerEntityId::random()),
            ..Default::default()
        };

        create_editor_entity.send(CreateEditorEntity(entity_def.clone()));

        level.entities.push(entity_def);
    }
}
