use crate::states::map_editor::menu::EditorInfo;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use shared::game::defs::{Defs, EntityType};
use shared::game::position::vec2_to_vec3;

pub fn path_lines(mut lines: ResMut<DebugLines>, defs: Res<Defs>, editor: Res<EditorInfo>) {
    let level_def = match defs.levels.get(&editor.map_name) {
        None => {
            return;
        }
        Some(m) => m,
    };

    for entity_def in level_def
        .entities
        .iter()
        .filter(|e| e.entity_type == EntityType::Path)
    {
        let path = entity_def
            .path
            .as_ref()
            .unwrap()
            .iter()
            .map(|p| vec2_to_vec3(&p.into()))
            .collect::<Vec<Vec3>>();
        let owner = entity_def.owner.as_ref().unwrap();
        for (a, b) in path.iter().zip(path.iter().skip(1)) {
            lines.line_colored(*a + Vec3::Y * 0.1, *b + Vec3::Y * 0.1, 0.0, owner.color());
        }
    }
}
