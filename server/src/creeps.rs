use crate::state::GameId;
use crate::{GameLookup, SpawnCreepsEvent, SpawnEntityEvent};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_math::Vec2;
use bevy_transform::prelude::*;
use naia_bevy_server::Server;
use shared::game::components::Speed;
use shared::game::defs::{EntityDef, EntityType};
use shared::game::owner::Owner;
use shared::game::path::{Path, PathProgress};
use shared::game::position::Position;
use shared::game::SpawnPoint;
use shared::protocol::Protocol;
use shared::Channels;

pub fn spawn_creeps(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    mut respawn_creeps_events: EventReader<SpawnCreepsEvent>,
    game_lookup: Res<GameLookup>,
) {
    for spawn_creep_event in respawn_creeps_events.iter() {
        let game_id = spawn_creep_event.game_id;
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

        let spawn_points = &game.spawn_points;
        if spawn_points.len() != 2 {
            warn!(
                "Not enough spawn points for game_id: {:?} for event: {:?}",
                game_id, spawn_creep_event
            );
            continue;
        }

        for (owner, position) in spawn_points {
            println!("Spawn point! {:?} {:?}", position, owner);

            // 20 creeps for each player, make a little grid.
            for x in 0..2 {
                for y in 0..2 {
                    let pos = *position + Vec2::new(x as f32 * 1.0, y as f32 * 1.0);
                    let spawn_entity_event = SpawnEntityEvent {
                        game_id,
                        entity_def: EntityDef {
                            entity_type: EntityType::Creep,
                            position: Some(pos.into()),
                            owner: Some(*owner),
                            creep: Some("robot".to_string()),
                            ..Default::default()
                        },
                    };
                    spawn_entity_events.send(spawn_entity_event);
                }
            }
        }
    }
}

pub fn move_along_path_and_optionally_tell_client(
    query: Query<(&mut Transform, &PathProgress, &Path, &GameId, &Speed)>,
    mut server: Server<Protocol, Channels>,
) {
    for (mut transform, path_progress, path, game_id) in query.iter() {
        let path_progress: &PathProgress = &path_progress;
        let distance = path_progress
            .previous_position
            .distance(path_progress.target_position);
        dbg!(distance);
        todo!();
    }
}
