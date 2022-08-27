use crate::net::ReleaseCreepEvent;
use bevy::prelude::*;
use shared::game::defs::CreepRef;
use shared::game::owner::Owner;
use shared::game::shared_game::{ServerEntityId, SharedGame};

#[derive(Component)]
pub struct Released(pub bool);

pub fn release_creeps(
    mut release_the_creeps_events: EventReader<ReleaseCreepEvent>,
    game: Query<&SharedGame>,
    mut creeps: Query<&mut Released>,
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

        if let Ok(mut released) = creeps.get_mut(*entity) {
            released.0 = true;
        } else {
            warn!("Could not get creeps component for entity {:?}", entity);
        }
    }
}
