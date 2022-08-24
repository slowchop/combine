use crate::release_creeps::send_message_to_game;
use crate::state::GameId;
use crate::{info, GameLookup, GameUserLookup, SpawnCreepsEvent, SpawnEntityEvent};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_math::{Vec2, Vec3};
use bevy_time::Time;
use bevy_transform::prelude::*;
use naia_bevy_server::Server;
use shared::game::components::Speed;
use shared::game::defs::{EntityDef, EntityType};
use shared::game::owner::Owner;
use shared::game::path::{Path, PathLeaveAt, PathProgress};
use shared::game::position::Position;
use shared::game::shared_game::ServerEntityId;
use shared::game::SpawnPoint;
use shared::protocol::update_position::UpdatePosition;
use shared::protocol::Protocol;
use shared::Channels;
use std::time::Duration;

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
            // for x in 0..2 {
            //     for y in 0..2 {
            let x = 0;
            let y = 0;
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
            // }
            // }
        }
    }
}

pub fn move_along_path_and_optionally_tell_client(
    mut commands: Commands,
    time: Res<Time>,
    game_user_lookup: Res<GameUserLookup>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut PathProgress,
        &ServerEntityId,
        &Path,
        &GameId,
        &Speed,
        Option<&PathLeaveAt>,
    )>,
    mut server: Server<Protocol, Channels>,
) {
    for (
        entity,
        mut transform,
        mut path_progress,
        server_entity_id,
        path,
        game_id,
        speed,
        path_leave_at,
    ) in query.iter_mut()
    {
        let mut need_to_broadcast = false;

        if let Some(path_leave_at) = path_leave_at {
            if time.time_since_startup() < path_leave_at.0 {
                println!("Creep shouldn't move yet!");
                continue;
            }

            commands.entity(entity).remove::<PathLeaveAt>();
            need_to_broadcast = true;
        }

        let mut movement_this_frame = speed.0 * time.delta_seconds();

        loop {
            let difference = (path_progress.target_position - transform.translation);

            let direction = difference.normalize();
            let distance_left = difference.length();

            // If we're still on the same path, move and break out.
            if movement_this_frame < distance_left {
                transform.translation += direction * movement_this_frame;
                break;
            }

            println!("Creep had hit a waypoint");
            movement_this_frame -= distance_left;
            path_progress.current_path_target += 1;
            if path_progress.current_path_target >= path.0.len() {
                // TODO: send event
                info!("We've hit the target!!!");
                todo!();
                break;
            }

            println!("Move on same path {:?}", transform.translation);
            path_progress.target_position = path.0[path_progress.current_path_target].clone();
            need_to_broadcast = true;
            // TODO: Tell client new position/velocity.
        }

        if need_to_broadcast {
            // TODO: VELOCITY!!!!!!
            let message =
                UpdatePosition::new(server_entity_id.clone(), transform.translation, Vec3::ZERO);
            send_message_to_game(
                &mut server,
                &game_id,
                &*game_user_lookup,
                Channels::ServerCommand,
                &message,
            );
        }
    }

    // let distance = path_progress
    //     .previous_position
    //     .distance(path_progress.target_position);
}
