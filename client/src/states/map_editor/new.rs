use crate::states::map_editor::menu::{ClearEditorLevelEvent, LoadEditorLevelEvent};
use crate::{Commands, Entity, EventReader, Query, With};
use bevy::prelude::*;
use shared::game::defs::{Defs, EntityDef, EntityType};
use shared::game::owner::Owner;

pub fn new_events(
    mut defs: ResMut<Defs>,
    mut commands: Commands,
    mut new_events: EventReader<ClearEditorLevelEvent>,
    mut load_events: EventWriter<LoadEditorLevelEvent>,
    query: Query<Entity, With<EntityDef>>,
) {
    for new_event in new_events.iter() {
        let level = defs.levels.entry(new_event.0.clone()).or_default();

        for entity in query.iter() {
            println!("Deleting entity {:?}", entity);
            commands.entity(entity).despawn();
        }

        let entity_def = EntityDef {
            entity_type: EntityType::Guide,
            position: Some(Vec2::ZERO.into()),
            ..Default::default()
        };
        level.entities.push(entity_def);

        let entity_def = EntityDef {
            entity_type: EntityType::Ground,
            position: Some(Vec2::ZERO.into()),
            texture: Some("ground.png".to_string()),
            ..Default::default()
        };
        level.entities.push(entity_def);

        for a in 0..2 {
            let owner = Owner::new(a);

            let mut path = Vec::new();
            for waypoints in 0..5 {
                path.push(Vec2::new((a as f32 - 1.0) * 1.0, waypoints as f32 * 1.0).into());
            }
            let entity_def = EntityDef {
                entity_type: EntityType::Path,
                path: Some(path),
                owner: Some(owner.clone()),
                ..Default::default()
            };
            level.entities.push(entity_def);

            let entity_def = EntityDef {
                entity_type: EntityType::Base,
                position: Some(Vec2::new((a as f32 - 1.0) * 1.0, -1.0).into()),
                owner: Some(owner.clone()),
                ..Default::default()
            };
            level.entities.push(entity_def);

            let entity_def = EntityDef {
                entity_type: EntityType::Spawn,
                position: Some(Vec2::new((a as f32 - 1.0) * 1.0, -5.0).into()),
                owner: Some(owner.clone()),
                ..Default::default()
            };
            level.entities.push(entity_def);
        }
        load_events.send(LoadEditorLevelEvent(new_event.0.clone()));
    }
}
