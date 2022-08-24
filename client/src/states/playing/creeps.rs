use crate::net::ReleaseCreepEvent;
use bevy::prelude::*;
use shared::game::defs::CreepRef;
use shared::game::owner::Owner;
use shared::game::shared_game::{ServerEntityId, SharedGame};

pub fn release_creeps(
    mut commands: Commands,
    mut release_the_creeps_events: EventReader<ReleaseCreepEvent>,
    game: Query<&SharedGame>,
    creeps: Query<&Owner, With<CreepRef>>,
) {
    // I've changed my mind. No client side creep release, just as a notification.
    // The creep positions will be sent from the server.

    return;

    // let game = match game.get_single() {
    //     Ok(g) => g,
    //     Err(_) => {
    //         warn!("Could not get game for release creeps");
    //         return;
    //     }
    // };
    //
    // for release_the_creep_event in release_the_creeps_events.iter() {
    //     println!("Release the creeps! {:?}", release_the_creep_event);
    //
    //     // Find creep by server id
    //     let entity =
    //         if let Some(entity) = game.entities.get(&release_the_creep_event.server_entity_id) {
    //             entity
    //         } else {
    //             warn!(
    //                 "Could not get entity for server_entity_id {:?}",
    //                 release_the_creep_event
    //             );
    //             continue;
    //         };
    //
    //     let owner = if let Ok(owner) = creeps.get(*entity) {
    //         owner
    //     } else {
    //         warn!("Could not get owner for entity {:?}", entity);
    //         continue;
    //     };
    //
    //     let path = if let Some(p) = game.paths.get(&owner) {
    //         p
    //     } else {
    //         warn!("Could not get path for owner {:?}", release_the_creep_event);
    //         continue;
    //     };

    // commands.entity(*entity).insert(path.clone());
    // }
}
