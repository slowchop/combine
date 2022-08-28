use crate::creeps::{ColdEffect, CreepNeedsPositionUpdate};
use crate::state::GameId;
use crate::GameLookup;
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_time::Time;
use bevy_transform::prelude::Transform;
use shared::game::defs::{CreepRef, Defs, TowerRef};
use shared::game::owner::Owner;
use shared::game::shared_game::ServerEntityId;
use shared::protocol::update_position::UpdatePosition;
use std::time::Duration;

#[derive(Component, Debug)]
pub struct LastShot(pub Duration);

#[derive(Debug)]
pub struct DamageCreepEvent {
    pub game_id: GameId,
    pub tower_id: Option<ServerEntityId>,
    pub tower_owner: Option<Owner>,
    pub creep_id: ServerEntityId,
    pub amount: u32,
}

pub fn shoot_towers(
    mut commands: Commands,
    time: Res<Time>,
    defs: Res<Defs>,
    game_lookup: Res<GameLookup>,
    mut towers: Query<(
        &TowerRef,
        &Transform,
        &mut LastShot,
        &GameId,
        &Owner,
        &ServerEntityId,
    )>,
    mut creeps: Query<(&CreepRef, &Transform, &Owner, &ServerEntityId)>,
    mut damage_creep_events: EventWriter<DamageCreepEvent>,
    mut creeps_that_need_position_update: EventWriter<CreepNeedsPositionUpdate>,
) {
    for (tower_ref, tower_transform, mut last_shot, game_id, tower_owner, server_entity_id) in
        towers.iter_mut()
    {
        let tower = if let Some(tower) = defs.tower(&tower_ref) {
            tower
        } else {
            warn!("Tower not found in defs: {:?}", tower_ref);
            continue;
        };

        let next_reload = last_shot.0 + Duration::from_secs_f32(tower.reload);
        let should_shoot = time.time_since_startup() >= next_reload;
        if !should_shoot {
            continue;
        }

        let game = if let Some(game) = game_lookup.0.get(game_id) {
            game
        } else {
            warn!(
                "Game not found in game_lookup for shoot_towers: {:?}",
                game_id
            );
            continue;
        };

        // Check if there are any towers in range. Maybe randomly run this to save CPU cycles.
        for (server_entity_id, creep_entity) in &game.entities {
            let (creep_ref, creep_transform, creep_owner, creep_server_entity_id) =
                if let Ok(c) = creeps.get(*creep_entity) {
                    c
                } else {
                    continue;
                };

            if tower_owner == creep_owner {
                continue;
            }

            let distance = creep_transform
                .translation
                .distance(tower_transform.translation);

            if distance > tower.range {
                continue;
            }

            println!(
                "Shooting from tower on team {:?} to a creep on team {:?}!",
                tower_owner, creep_owner
            );
            last_shot.0 = time.time_since_startup().clone();

            damage_creep_events.send(DamageCreepEvent {
                game_id: *game_id,
                tower_id: Some(*server_entity_id),
                tower_owner: Some(*tower_owner),
                creep_id: *creep_server_entity_id,
                amount: tower.instant_damage,
            });

            if let Some(cold_slowdown_duration) = tower.cold_slowdown_duration {
                commands.entity(*creep_entity).insert(ColdEffect {
                    until: time.time_since_startup()
                        + Duration::from_secs_f32(cold_slowdown_duration),
                    amount: tower.cold_slowdown_amount.unwrap(),
                });

                creeps_that_need_position_update
                    .send(CreepNeedsPositionUpdate(*creep_server_entity_id));
            }

            break;
        }
    }
}
