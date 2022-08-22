use bevy_ecs::prelude::*;
use shared::game::defs::EntityDef;

#[derive(Debug, Clone)]
pub struct SpawnServerEntity(pub EntityDef);

pub fn spawn_entities(mut commands: Commands, spawn_entities: EventReader<SpawnServerEntity>) {
    for spawn in spawn_entities.iter() {
        let entity_def = spawn.0;
        let mut spawned = commands.spawn();
    }
}
