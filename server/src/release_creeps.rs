use crate::state::GameId;
use crate::{GameLookup, GameUserLookup, ReleaseCreepsEvent};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_time::Time;
use bevy_transform::prelude::*;
use naia_bevy_server::shared::{ChannelIndex, Protocolize, ReplicateSafe};
use naia_bevy_server::{Server, UserKey};
use rand::{thread_rng, Rng};
use shared::game::components::Speed;
use shared::game::defs::CreepRef;
use shared::game::owner::Owner;
use shared::game::path::{Path, PathLeaveAt, PathProgress};
use shared::game::position::{vec2_to_vec3, Position};
use shared::game::shared_game::{ServerEntityId, SharedGame};
use shared::protocol::release_creep::ReleaseCreeps;
use shared::protocol::Protocol;
use shared::Channels;
use std::ops::Add;
use std::time::Duration;

pub fn tell_clients_to_release_the_creeps(
    time: Res<Time>,
    mut commands: Commands,
    mut release_creeps_events: EventReader<ReleaseCreepsEvent>,
    mut server: Server<Protocol, Channels>,
    game_user_lookup: Res<GameUserLookup>,
    game_lookup: Res<GameLookup>,
    creep_query: Query<
        (&ServerEntityId, &Transform, &Owner, &Speed),
        (With<CreepRef>, Without<Path>),
    >,
) {
    for release_creeps_event in release_creeps_events.iter() {
        let users = match game_user_lookup.get_game_users(&release_creeps_event.game_id) {
            Some(u) => u,
            None => {
                warn!(
                    "Could not get users for game_id {:?} for release creeps",
                    release_creeps_event
                );
                continue;
            }
        };
        if users.len() == 0 {
            warn!(
                "Could not get game_user_lookup for game_id {:?} for release creeps",
                release_creeps_event
            );
            continue;
        }

        let game = match game_lookup.0.get(&release_creeps_event.game_id) {
            Some(g) => g,
            None => {
                warn!(
                    "Could not get game for game_id {:?} for release creeps",
                    release_creeps_event
                );
                continue;
            }
        };
        if game.winner.is_some() {
            continue;
        }

        // Iterate over entities in the game.
        // Work out which ones are creeps.
        // Send a ReleaseCreep message to each client for each entity.
        let mut collected_server_entity_ids = Vec::with_capacity(40);
        for (idx, (server_entity_id, entity)) in game.entities.iter().enumerate() {
            let (server_entity_id_2, transform, owner, speed) = match creep_query.get(*entity) {
                Ok(e) => e,
                Err(_) => {
                    warn!(
                        "Could not get creep for entity while trying to release: {:?}",
                        entity
                    );
                    continue;
                }
            };
            debug_assert_eq!(server_entity_id, server_entity_id_2);

            let path = if let Some(p) = game.paths.get(owner) {
                p
            } else {
                warn!("Could not get path for owner {:?}", owner);
                continue;
            };

            collected_server_entity_ids.push(*server_entity_id);

            commands
                .entity(*entity)
                .insert(path.clone())
                .insert(PathProgress {
                    target_position: path.0[0],
                    current_path_target: 0,
                })
                .insert(PathLeaveAt(
                    time.time_since_startup() + Duration::from_secs_f32(idx as f32 / speed.0),
                ));
        }

        // We're only sending one "release" message to each client. The server will send position
        // updates to the client for the creeps.
        let multiplier = game.multipliers();
        let message = ReleaseCreeps::new(collected_server_entity_ids);
        send_message_to_game(
            &mut server,
            &*game_user_lookup,
            &release_creeps_event.game_id,
            Channels::ServerCommand,
            &message,
        );
    }
}

pub fn send_message_to_game<R, P>(
    server: &mut Server<Protocol, Channels>,
    game_user_lookup: &GameUserLookup,
    game_id: &GameId,
    channel: Channels,
    message: &R,
) where
    R: ReplicateSafe<P> + ReplicateSafe<Protocol>,
    P: Protocolize,
{
    let users = match game_user_lookup.get_game_users(game_id) {
        Some(u) => u,
        None => {
            warn!(
                "Could not get users for game_id {:?} for sending a message",
                game_id
            );
            return;
        }
    };
    if users.len() == 0 {
        warn!(
            "Could not get enough users for game_id {:?} for sending message",
            game_id,
        );
        return;
    }

    for user_key in users {
        server.send_message(user_key, channel, message);
    }
}
