use crate::{DamageCreepEvent, DestroyEntityEvent, GameLookup};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use shared::game::defs::{CreepRef, Defs};
use shared::game::destroyment_method::DestroymentMethod;

#[derive(Component, Debug)]
pub struct Damaged(pub f32);

pub fn damage_creeps(
    defs: Res<Defs>,
    mut commands: Commands,
    mut damage_creep_events: EventReader<DamageCreepEvent>,
    game_lookup: Res<GameLookup>,
    mut creeps: Query<(&mut Damaged, &CreepRef)>,
    mut destroy_entity_events: EventWriter<DestroyEntityEvent>,
) {
    for damage_creep_event in damage_creep_events.iter() {
        let game = if let Some(game) = game_lookup.0.get(&damage_creep_event.game_id) {
            game
        } else {
            warn!(
                "Game not found in game_lookup for damage_creeps: {:?}",
                damage_creep_event.game_id
            );
            continue;
        };

        let entity = if let Some(entity) = game.entities.get(&damage_creep_event.server_entity_id) {
            entity
        } else {
            warn!(
                "Entity not found in game.entities for damage_creeps: {:?}",
                damage_creep_event.server_entity_id
            );
            continue;
        };

        let (mut damaged, creep_ref) = if let Ok(c) = creeps.get_mut(*entity) {
            c
        } else {
            warn!(
                "Entity not found in creeps for damage_creeps: {:?}",
                damage_creep_event.server_entity_id
            );
            continue;
        };

        let creep = if let Some(creep) = defs.creep(&creep_ref.0) {
            creep
        } else {
            warn!("Creep not found in defs: {:?}", creep_ref);
            continue;
        };

        damaged.0 += damage_creep_event.amount;
        // TODO: Emit "TowerShooting"

        println!("damaged creep: {:?}", damaged.0);

        if damaged.0 < creep.health {
            continue;
        }

        println!("dead creep: {:?}", damaged.0);

        destroy_entity_events.send(DestroyEntityEvent {
            game_id: damage_creep_event.game_id,
            server_entity_id: damage_creep_event.server_entity_id,
            destroyment_method: DestroymentMethod::Quiet,
        });
    }
}
