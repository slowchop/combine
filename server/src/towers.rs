use crate::state::GameId;
use crate::GameLookup;
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_time::Time;
use bevy_transform::prelude::Transform;
use shared::game::defs::{CreepRef, Defs, TowerRef};
use shared::game::owner::Owner;
use std::time::Duration;

#[derive(Component, Debug)]
pub struct LastShot(Duration);

pub fn shoot_towers(
    time: Res<Time>,
    game_lookup: Res<GameLookup>,
    mut towers: Query<(
        &TowerRef,
        &Transform,
        Option<&mut LastShot>,
        &GameId,
        &Owner,
    )>,
    mut creeps: Query<(&CreepRef, &Transform, &Owner)>,
    defs: Res<Defs>,
) {
    for (tower_ref, tower_transform, maybe_last_shot, game_id, tower_owner) in towers.iter_mut() {
        let tower = if let Some(tower) = defs.tower(&tower_ref.0) {
            tower
        } else {
            warn!("Tower not found in defs: {:?}", tower_ref);
            continue;
        };

        let should_shoot = if let Some(last_shot) = maybe_last_shot {
            let next_reload = last_shot.0 + Duration::from_secs_f32(tower.reload);
            time.time_since_startup() >= next_reload
        } else {
            true
        };

        if !should_shoot {
            continue;
        }

        println!("should shoot");

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
        for (server_entity_id, entity) in &game.entities {
            println!("searching for creeps in game.entities");

            let (creep_ref, creep_transform, creep_owner) = if let Ok(c) = creeps.get(*entity) {
                c
            } else {
                continue;
            };

            println!("found maybe creep to shoot");

            if tower_owner == creep_owner {
                continue;
            }

            println!("found enemy creep to shoot");

            let distance = creep_transform
                .translation
                .distance(tower_transform.translation);

            println!("Distance: {}", distance);

            if distance > tower.range {
                continue;
            }

            panic!("found a creep to hit!")
        }
    }
}
