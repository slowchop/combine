use crate::creeps::ColdEffect;
use crate::damage::Damaged;
use crate::new_entities::NewEntityEvent;
use crate::state::GameId;
use crate::towers::LastShot;
use crate::GameLookup;
use bevy_ecs::prelude::*;
use bevy_log::{error, info, warn};
use bevy_math::{Vec2, Vec3};
use bevy_transform::prelude::Transform;
use shared::game::components::Speed;
use shared::game::defs::{CreepRef, Defs, EntityDef, EntityType, TowerRef};
use shared::game::owner::Owner;
use shared::game::path::Path;
use shared::game::position::{vec2_to_vec3, Position};
use shared::game::SpawnPoint;
use std::time::Duration;

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
        if game.winner.is_some() {
            warn!("Spawn disabled because game is over.");
            continue;
        }

        let mut created_entity = None;
        match entity_def.entity_type {
            // Ignore these.
            EntityType::Guide => {}
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
            EntityType::Path => {
                let owner = match entity_def.owner {
                    Some(o) => o,
                    None => {
                        error!("path Spawn entity has no owner!");
                        continue;
                    }
                };

                let path = match &entity_def.path {
                    Some(p) => p,
                    None => {
                        error!("path Spawn entity has no path!");
                        continue;
                    }
                };

                info!("Inserting path for owner {:?}", owner);
                let path: Vec<Vec3> = path.iter().map(|p| vec2_to_vec3(&p.into())).collect();
                game.paths.insert(owner, Path(path));
            }
            EntityType::Tower => {
                let tower_ref = match &entity_def.tower {
                    Some(t) => t,
                    None => {
                        warn!("Tower entity has no tower name: {:?}", entity_def);
                        continue;
                    }
                };
                let _tower = match defs.tower(tower_ref) {
                    Some(t) => t,
                    None => {
                        warn!("Tower not found: {:?} {:?}", tower_ref, entity_def);
                        continue;
                    }
                };

                let position = match &entity_def.position {
                    Some(p) => p,
                    None => {
                        warn!("Spawn entity has no position: {:?}", entity_def);
                        continue;
                    }
                };
                let position: Position = position.0.into();
                let owner = match entity_def.owner {
                    Some(o) => o,
                    None => {
                        warn!("Spawn entity has no owner: {:?}", entity_def);
                        continue;
                    }
                };
                debug_assert!(owner != Owner::waiting());

                let tower = match &entity_def.tower {
                    Some(t) => t,
                    None => {
                        warn!("Spawn entity has no tower!: {:?}", entity_def);
                        continue;
                    }
                };

                let id = commands
                    .spawn()
                    .insert(Transform::from_translation(position.0))
                    .insert(tower.clone())
                    .insert(owner)
                    .insert(game_id)
                    .insert(LastShot(Duration::ZERO))
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
                let position: Position = position.0.into();
                let owner = match entity_def.owner {
                    Some(o) => o,
                    None => {
                        warn!("Spawn entity has no owner: {:?}", entity_def);
                        continue;
                    }
                };
                let creep_ref = match &entity_def.creep {
                    Some(t) => t,
                    None => {
                        warn!("Spawn entity has no creep!: {:?}", entity_def);
                        continue;
                    }
                };
                let creep = match defs.creep(creep_ref) {
                    Some(t) => t,
                    None => {
                        warn!("Creep not found: {:?} {:?}", creep_ref, entity_def);
                        continue;
                    }
                };

                let id = commands
                    .spawn()
                    .insert(Transform::from_translation(position.0))
                    .insert(creep_ref.clone())
                    .insert(owner)
                    .insert(game_id)
                    .insert(Speed(creep.speed))
                    .insert(Damaged(0))
                    .insert(ColdEffect::zero())
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
