use crate::states::map_editor::menu::EditorInfo;
use crate::states::playing::debug_lines::circle;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use shared::game::defs::{Defs, EntityType};
use shared::game::position::vec2_to_vec3;

pub fn editor_lines(mut lines: ResMut<DebugLines>, defs: Res<Defs>, editor: Res<EditorInfo>) {
    let level_def = match defs.levels.get(&editor.map_name) {
        None => {
            return;
        }
        Some(m) => m,
    };

    for entity_def in &level_def.entities {
        match entity_def.entity_type {
            EntityType::Path => {
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
            EntityType::BuildableCircle => {
                let net_vec2 = entity_def.position.as_ref().unwrap().clone();
                // println!("BuildableCircle {:?}", net_vec2);
                let position = vec2_to_vec3(&net_vec2.into()) + Vec3::Y * 0.1;
                let radius = entity_def.radius.as_ref().unwrap();
                let owner = entity_def.owner.as_ref().unwrap();
                circle(&mut lines, position, *radius, 10, owner.color());
            }
            _ => {}
        }
    }
}
