use crate::state::GameId;
use crate::{GameLookup, GameUserLookup, ReleaseCreepsEvent};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_time::Time;
use bevy_transform::prelude::*;
use naia_bevy_server::shared::{ChannelIndex, Protocolize, ReplicateSafe};
use naia_bevy_server::{Server, UserKey};
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
    creep_query: Query<(&ServerEntityId, &Transform, &Owner), (With<CreepRef>, Without<Path>)>,
) {
    for release_creeps_event in release_creeps_events.iter() {
        println!("Trying to sending release creeps event!");
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

        // Iterate over entities in the game.
        // Work out which ones are creeps.
        // Send a ReleaseCreep message to each client for each entity.
        for (server_entity_id, entity) in &game.entities {
            let (server_entity_id_2, transform, owner) = match creep_query.get(*entity) {
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

            dbg!(&game.paths.len());

            let path = if let Some(p) = game.paths.get(owner) {
                p
            } else {
                warn!("Could not get path for owner {:?}", owner);
                continue;
            };

            println!("Inserting path components to creep {:?}", &path);
            commands
                .entity(*entity)
                .insert(path.clone())
                .insert(PathProgress {
                    target_position: path.0[0],
                    current_path_target: 0,
                })
                .insert(PathLeaveAt(
                    time.time_since_startup() + Duration::from_secs(2),
                ));
        }

        // We're only sending one "release" message to each client. The server will send position
        // updates to the client for the creeps.
        let message = ReleaseCreeps::new();
        send_message_to_game(
            &mut server,
            &release_creeps_event.game_id,
            &*game_user_lookup,
            Channels::ServerCommand,
            &message,
        );
    }
}

pub fn send_message_to_game<R, P>(
    server: &mut Server<Protocol, Channels>,
    game_id: &GameId,
    game_user_lookup: &GameUserLookup,
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
