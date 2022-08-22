use bevy_ecs::prelude::*;
use bevy_log::warn;
use shared::game::defs::{EntityDef, EntityType};

#[derive(Debug, Clone)]
pub struct SpawnServerEntityEvent(pub EntityDef);

pub fn spawn_entities(
    mut commands: Commands,
    mut spawn_entities: EventReader<SpawnServerEntityEvent>,
) {
    for spawn in spawn_entities.iter() {
        let entity_def = &spawn.0;
        let mut spawned = commands.spawn();

        match entity_def.entity_type {
            // EntityType::Sprite => {}
            // EntityType::Ground => {}
            // EntityType::Spawn => {}
            // EntityType::Base => {}
            // EntityType::Path => {}
            // EntityType::Tower => {}
            // EntityType::Creep => {}
            _ => {
                warn!("no spawn for entity {:?}", entity_def);
                continue;
            }
        }
    }
}
