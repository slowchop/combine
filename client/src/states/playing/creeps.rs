use crate::net::ReleaseCreepEvent;
use bevy::prelude::*;
use shared::game::defs::CreepRef;
use shared::game::owner::Owner;
use shared::game::shared_game::{ServerEntityId, SharedGame};

#[derive(Component)]
pub struct Released;

pub fn release_creeps(
    mut commands: Commands,
    mut release_the_creeps_events: EventReader<ReleaseCreepEvent>,
    game: Query<&SharedGame>,
) {
    // No client side creep release, just as a notification.
    // The creep positions will be sent from the server.

    let game = match game.get_single() {
        Ok(g) => g,
        Err(_) => {
            warn!("Could not get game for release creeps");
            return;
        }
    };

    for release_the_creep_event in release_the_creeps_events.iter() {
        let entity =
            if let Some(entity) = game.entities.get(&release_the_creep_event.server_entity_id) {
                entity
            } else {
                warn!(
                    "Could not get creep entity for server_entity_id {:?}",
                    release_the_creep_event
                );
                continue;
            };

        commands.entity(*entity).insert(Released);
    }
}
