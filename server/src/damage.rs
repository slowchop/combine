use crate::release_creeps::send_message_to_game;
use crate::{DamageCreepEvent, DestroyEntityEvent, GameLookup, GameUserLookup};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use naia_bevy_server::Server;
use shared::game::components::MaxHealth;
use shared::game::defs::{CreepRef, Defs};
use shared::game::destroyment_method::DestroymentMethod;
use shared::protocol::hurt_entity::HurtEntity;
use shared::protocol::Protocol;
use shared::Channels;

#[derive(Component, Debug)]
pub struct Damaged(pub u32);

pub fn damage_creeps(
    defs: Res<Defs>,
    mut commands: Commands,
    mut damage_creep_events: EventReader<DamageCreepEvent>,
    game_user_lookup: Res<GameUserLookup>,
    game_lookup: Res<GameLookup>,
    mut creeps: Query<(&mut Damaged, &MaxHealth, &CreepRef)>,
    mut destroy_entity_events: EventWriter<DestroyEntityEvent>,
    mut server: Server<Protocol, Channels>,
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

        let entity = if let Some(entity) = game.entities.get(&damage_creep_event.creep_id) {
            entity
        } else {
            warn!(
                "Entity not found in game.entities for damage_creeps: {:?}",
                damage_creep_event.creep_id
            );
            continue;
        };

        let (mut damaged, max_health, creep_ref) = if let Ok(c) = creeps.get_mut(*entity) {
            c
        } else {
            warn!(
                "Entity not found in creeps for damage_creeps: {:?}",
                damage_creep_event.creep_id
            );
            continue;
        };

        let creep = if let Some(creep) = defs.creep(&creep_ref) {
            creep
        } else {
            warn!("Creep not found in defs: {:?}", creep_ref);
            continue;
        };

        damaged.0 += damage_creep_event.amount;

        println!("damaged creep: {:?}", damaged.0);

        let message = HurtEntity::new(
            damage_creep_event.tower_id,
            damage_creep_event.creep_id,
            damaged.0,
        );
        send_message_to_game(
            &mut server,
            &game_user_lookup,
            &damage_creep_event.game_id,
            Channels::ServerCommand,
            &message,
        );

        if damaged.0 < max_health.0 {
            continue;
        }

        let (gold_earned, gold_earned_for) = if let Some(o) = damage_creep_event.tower_owner {
            ((max_health.0 as f32 * 0.25) as u32, Some(o))
        } else {
            (0, None)
        };

        destroy_entity_events.send(DestroyEntityEvent {
            game_id: damage_creep_event.game_id,
            server_entity_id: damage_creep_event.creep_id,
            destroyment_method: DestroymentMethod::Quiet,
            gold_earned,
            gold_earned_for,
        });
    }
}
