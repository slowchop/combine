use crate::new_entities::NewEntityEvent;
use crate::state::GameId;
use crate::GameLookup;
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use shared::game::defs::{EntityDef, EntityType, TowerRef};
use shared::game::owner::Owner;
use shared::game::SpawnPoint;
use shared::protocol::position::Position;

#[derive(Debug, Clone)]
pub struct SpawnServerEntityEvent {
    pub game_id: GameId,
    pub entity_def: EntityDef,
}

pub fn spawn_entities(
    mut commands: Commands,
    mut spawn_entities: EventReader<SpawnServerEntityEvent>,
    mut new_entity_events: EventWriter<NewEntityEvent>,
) {
    for spawn in spawn_entities.iter() {
        let entity_def = &spawn.entity_def;
        let game_id = spawn.game_id;

        let mut created_entity = None;
        match entity_def.entity_type {
            // Ignore these.
            EntityType::Sprite => {}
            EntityType::Ground => {}
            EntityType::Base => {}

            EntityType::Spawn => {
                let position = match &entity_def.position {
                    Some(p) => p,
                    None => {
                        warn!("Spawn entity has no position!");
                        continue;
                    }
                };
                let owner = match entity_def.owner {
                    Some(o) => o,
                    None => {
                        warn!("Spawn entity has no owner!");
                        continue;
                    }
                };

                let id = commands
                    .spawn()
                    .insert(Position::new(position.into()))
                    .insert(SpawnPoint)
                    .insert(owner)
                    .insert(game_id)
                    .id();
                created_entity = Some(id);

                info!("Spawned SpawnPoint at {:?}", position);
            }
            EntityType::Path => {}
            EntityType::Tower => {
                let position = match &entity_def.position {
                    Some(p) => p,
                    None => {
                        warn!("Spawn entity has no position: {:?}", entity_def);
                        continue;
                    }
                };
                let owner = match entity_def.owner {
                    Some(o) => o,
                    None => {
                        warn!("Spawn entity has no owner: {:?}", entity_def);
                        continue;
                    }
                };
                let tower = match &entity_def.tower {
                    Some(t) => t,
                    None => {
                        warn!("Spawn entity has no tower!: {:?}", entity_def);
                        continue;
                    }
                };

                let id = commands
                    .spawn()
                    .insert(Position::new(position.into()))
                    .insert(TowerRef(tower.clone()))
                    .insert(owner)
                    .insert(game_id)
                    .id();

                created_entity = Some(id);
            }
            EntityType::Creep => {}
            _ => {
                warn!("no spawn for entity {:?}", entity_def);
                continue;
            }
        }

        if let Some(entity) = created_entity {
            new_entity_events.send(NewEntityEvent {
                game_id,
                entity,
                entity_def: entity_def.clone(),
            });
        }
    }
}
