use crate::states::editor::input_events::Draggable;
use crate::states::editor::load_map::PathInfo;
use crate::states::editor::menu::{EditorInfo, SaveEvent};
use bevy::prelude::*;
use bevy::utils::HashMap;
use shared::game::defs::{Defs, EntityDef, EntityType};
use shared::game::owner::Owner;
use shared::game::position::vec3_to_vec2;

pub fn save_map(
    mut save_map_events: EventReader<SaveEvent>,
    mut commands: Commands,
    mut defs: ResMut<Defs>,
    editor_info: Res<EditorInfo>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            Option<&mut EntityDef>,
            Option<&mut PathInfo>,
        ),
        With<Draggable>,
    >,
) {
    for event in save_map_events.iter() {
        // Normal entities (not paths) will have an EntityDef component which we will just nuke
        // the existing entities with.
        // PathInfos will be sorted into owner and index and will also nuke the existing path
        // entities.

        let map_name = event.0.clone();
        let level_def = defs
            .levels
            .entry(map_name.clone())
            .or_insert_with(Default::default);

        let mut new_entities = Vec::new();
        let mut new_paths = HashMap::new();
        new_paths.insert(Owner::new(0), Vec::new());
        new_paths.insert(Owner::new(1), Vec::new());
        for (entity, transform, maybe_entity_def, maybe_path_info) in query.iter() {
            if let Some(entity_def) = maybe_entity_def {
                new_entities.push(entity_def.clone());
            } else if let Some(path_info) = maybe_path_info {
                let owner = path_info.owner;
                new_paths
                    .get_mut(&owner)
                    .unwrap()
                    .push((path_info.index, transform.translation));
            } else {
                warn!("Unknown entity {:?} {:?}", entity, transform);
                continue;
            }
        }

        for (owner, path) in new_paths.iter() {
            let mut path = path.clone();
            path.sort_by_key(|(index, _)| *index);
            let path = path
                .into_iter()
                .map(|(_, pos)| vec3_to_vec2(&pos).into())
                .collect();
            new_entities.push(EntityDef {
                entity_type: EntityType::Path,
                owner: Some(owner.clone()),
                path: Some(path),
                ..Default::default()
            });
        }

        level_def.name = map_name.clone();
        level_def.entities = new_entities;
        defs.save();
    }
}
