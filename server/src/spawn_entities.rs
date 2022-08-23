use crate::new_entities::NewEntityEvent;
use crate::state::GameId;
use crate::GameLookup;
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use shared::game::defs::{CreepRef, Defs, EntityDef, EntityType, TowerRef};
use shared::game::owner::Owner;
use shared::game::position::Position;
use shared::game::SpawnPoint;

#[derive(Debug, Clone)]
pub struct SpawnEntityEvent {
    pub game_id: GameId,
    pub entity_def: EntityDef,
}

pub fn spawn_entities(
    mut commands: Commands,
    mut spawn_entities: EventReader<SpawnEntityEvent>,
    mut new_entity_events: EventWriter<NewEntityEvent>,
    mut game_lookup: ResMut<GameLookup>,
    defs: Res<Defs>,
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
                let owner = match entity_def.owner {
                    Some(o) => o,
                    None => {
                        warn!("Spawn entity has no owner!");
                        continue;
                    }
                };

                let mut game = match game_lookup.0.get_mut(&game_id) {
                    Some(g) => g,
                    None => {
                        warn!(
                            "Could not get game for game_id {:?} for {:?}",
                            game_id, spawn
                        );
                        continue;
                    }
                };
                let position = match &entity_def.position {
                    Some(p) => p,
                    None => {
                        warn!("Spawn entity has no position!");
                        continue;
                    }
                };

                game.spawn_points.insert(owner, position.into());

                // We don't need to create spawn points on the server because we're just going
                // to track their locations in SharedGame.
                //
                // Also don't need to send them to the client because they have them on their map.

                // let id = commands
                //     .spawn()
                //     .insert(Position(position.into()))
                //     .insert(SpawnPoint)
                //     .insert(owner)
                //     .insert(game_id)
                //     .id();
                // created_entity = Some(id);
                //
                // info!("Spawned SpawnPoint at {:?}", position);
            }
            EntityType::Path => {}
            EntityType::Tower => {
                let tower_name = match &entity_def.tower {
                    Some(t) => t,
                    None => {
                        warn!("Tower entity has no tower name: {:?}", entity_def);
                        continue;
                    }
                };
                let tower = match defs.tower(tower_name.as_str()) {
                    Some(t) => t,
                    None => {
                        warn!("Tower not found: {:?} {:?}", tower_name, entity_def);
                        continue;
                    }
                };
                // TODO: tower will be used for attributes.

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
                    .insert(Position(position.into()))
                    .insert(TowerRef(tower.clone()))
                    .insert(owner)
                    .insert(game_id)
                    .id();

                created_entity = Some(id);
            }
            EntityType::Creep => {
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
                let creep = match &entity_def.creep {
                    Some(t) => t,
                    None => {
                        warn!("Spawn entity has no creep!: {:?}", entity_def);
                        continue;
                    }
                };

                let id = commands
                    .spawn()
                    .insert(Position(position.into()))
                    .insert(CreepRef(creep.clone()))
                    .insert(owner)
                    .insert(game_id)
                    .id();

                created_entity = Some(id);
            }
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
