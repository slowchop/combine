use crate::release_creeps::send_message_to_game;
use crate::state::GameId;
use crate::stats::LoseALifeEvent;
use crate::{
    info, DestroyEntityEvent, GameLookup, GameUserLookup, SpawnCreepsEvent, SpawnEntityEvent,
};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_math::{Vec2, Vec3};
use bevy_time::Time;
use bevy_transform::prelude::*;
use bevy_utils::HashSet;
use naia_bevy_server::Server;
use shared::game::components::Speed;
use shared::game::defs::{Creep, CreepRef, EntityDef, EntityType};
use shared::game::destroyment_method::DestroymentMethod;
use shared::game::owner::Owner;
use shared::game::path::{Path, PathLeaveAt, PathProgress};
use shared::game::position::Position;
use shared::game::shared_game::ServerEntityId;
use shared::game::SpawnPoint;
use shared::protocol::destroy_entity::DestroyEntity;
use shared::protocol::update_position::UpdatePosition;
use shared::protocol::Protocol;
use shared::Channels;
use std::time::Duration;

#[derive(Component, Debug)]
pub struct ColdEffect {
    pub until: Duration,
    /// Creep speed becomes their normal speed minus this amount.
    pub amount: f32,
}

impl ColdEffect {
    pub fn zero() -> Self {
        Self {
            until: Duration::from_micros(0),
            amount: 0.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct FireEffect {
    pub until: Duration,
    pub interval: Duration,
    pub amount: f32,
}

pub fn spawn_creeps(
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,
    mut respawn_creeps_events: EventReader<SpawnCreepsEvent>,
    mut game_lookup: ResMut<GameLookup>,
) {
    for spawn_creep_event in respawn_creeps_events.iter() {
        let game_id = spawn_creep_event.game_id;
        let game = match game_lookup.0.get_mut(&game_id) {
            Some(game) => game,
            None => {
                warn!(
                    "Game not found for game_id: {:?} for event: {:?}",
                    game_id, spawn_creep_event
                );
                continue;
            }
        };
        if game.winner.is_some() {
            continue;
        }

        let spawn_points = &game.spawn_points;
        if spawn_points.len() != 2 {
            warn!(
                "Not enough spawn points for game_id: {:?} for event: {:?}",
                game_id, spawn_creep_event
            );
            continue;
        }

        for (owner, position) in spawn_points {
            // 16 creeps for each player, make a little grid.
            let mut i = 0;
            for x in 0..4 {
                for y in 0..4 {
                    i += 1;

                    let creep_ref = if i % 2 == 0 {
                        CreepRef("robot".to_string())
                    } else {
                        CreepRef("monster".to_string())
                    };

                    let pos = *position + Vec2::new((x - 2) as f32 * 3.0, (y - 2) as f32 * 3.0);
                    let multiplier = game.multipliers();
                    let spawn_entity_event = SpawnEntityEvent {
                        game_id,
                        entity_def: EntityDef {
                            entity_type: EntityType::Creep,
                            position: Some(pos.into()),
                            owner: Some(*owner),
                            creep: Some(creep_ref),
                            ..Default::default()
                        },
                        speed_multiplier: multiplier.speed,
                        health_multiplier: multiplier.health,
                    };

                    spawn_entity_events.send(spawn_entity_event);
                }
            }
        }
    }
}

pub struct CreepNeedsPositionUpdate(pub ServerEntityId);

pub fn move_along_path(
    mut commands: Commands,
    time: Res<Time>,
    game_user_lookup: Res<GameUserLookup>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut PathProgress,
        &Owner,
        &ServerEntityId,
        &Path,
        &GameId,
        &Speed,
        Option<&PathLeaveAt>,
        &ColdEffect,
    )>,
    mut server: Server<Protocol, Channels>,
    mut destroy_entity_events: EventWriter<DestroyEntityEvent>,
    mut lose_a_life_events: EventWriter<LoseALifeEvent>,
    mut creeps_that_need_position_update: EventReader<CreepNeedsPositionUpdate>,
) {
    let mut creep_ids_that_need_position_update = HashSet::new();
    for CreepNeedsPositionUpdate(creep_id) in creeps_that_need_position_update.iter() {
        creep_ids_that_need_position_update.insert(*creep_id);
    }

    for (
        entity,
        mut transform,
        mut path_progress,
        owner,
        server_entity_id,
        path,
        game_id,
        speed,
        path_leave_at,
        cold_effect,
    ) in query.iter_mut()
    {
        let mut need_to_broadcast = false;
        if creep_ids_that_need_position_update.contains(server_entity_id) {
            need_to_broadcast = true;
        }

        if let Some(path_leave_at) = path_leave_at {
            if time.time_since_startup() < path_leave_at.0 {
                continue;
            }

            commands.entity(entity).remove::<PathLeaveAt>();
            need_to_broadcast = true;
        }

        let mut speed = speed.0;
        // Cold
        if time.time_since_startup() <= cold_effect.until {
            speed -= cold_effect.amount;
        }

        if speed < 0. {
            speed = 0.;
        }

        // Reduce this for each new path we take. Usually should be 0 or 1 times!
        let mut movement_this_frame = speed * time.delta_seconds();

        // Final velocity after any paths turned.
        let mut velocity = Vec3::ZERO;

        loop {
            let difference = (path_progress.target_position - transform.translation);
            let direction = difference.normalize();
            let distance_left = difference.length();

            // If we're still on the same path, move and break out.
            if movement_this_frame < distance_left {
                transform.translation += direction * movement_this_frame;
                velocity = direction * speed;
                break;
            }

            movement_this_frame -= distance_left;
            path_progress.current_path_target += 1;
            if path_progress.current_path_target >= path.0.len() {
                lose_a_life_events.send(LoseALifeEvent {
                    game_id: *game_id,
                    who: owner.other_player(),
                });

                destroy_entity_events.send(DestroyEntityEvent {
                    game_id: *game_id,
                    server_entity_id: *server_entity_id,
                    destroyment_method: DestroymentMethod::Quiet,
                    gold_earned: 0,
                    gold_earned_for: None,
                });

                break;
            }

            path_progress.target_position = path.0[path_progress.current_path_target].clone();
            need_to_broadcast = true;
        }

        if need_to_broadcast {
            let message =
                UpdatePosition::new(server_entity_id.clone(), transform.translation, velocity);
            send_message_to_game(
                &mut server,
                &*game_user_lookup,
                &game_id,
                Channels::ServerCommand,
                &message,
            );
        }
    }
}
