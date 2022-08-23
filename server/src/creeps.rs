use crate::{GameLookup, SpawnCreepsEvent, SpawnEntityEvent};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use shared::game::defs::{EntityDef, EntityType};
use shared::game::owner::Owner;
use shared::game::position::Position;
use shared::game::SpawnPoint;

pub fn spawn_creeps(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    mut respawn_creeps_events: EventReader<SpawnCreepsEvent>,
    game_lookup: Res<GameLookup>,
    spawn_points: Query<(&Owner, &Position), With<SpawnPoint>>,
) {
    for spawn_creep_event in respawn_creeps_events.iter() {
        let game_id = spawn_creep_event.0;
        let game = match game_lookup.0.get(&game_id) {
            Some(game) => game,
            None => {
                warn!(
                    "Game not found for game_id: {:?} for event: {:?}",
                    game_id, spawn_creep_event
                );
                continue;
            }
        };

        // Find the spawn points for this game.
        for entity in game.entities.values() {
            // TODO: Maybe slow
            let (owner, position) = match spawn_points.get(*entity) {
                Ok(s) => s,
                Err(_) => continue,
            };

            println!("Found spawn point! {:?} {:?}", position, owner);

            let spawn_entity_event = SpawnEntityEvent {
                game_id,
                entity_def: EntityDef {
                    entity_type: EntityType::Creep,
                    position: Some(position.0.into()),
                    owner: Some(*owner),
                    creep: Some("robot".to_string()),
                    ..Default::default()
                },
            };
            spawn_entity_events.send(spawn_entity_event);
        }
    }
}
