use crate::state::PlayerQueue;
use crate::state::{GameId, PlayerLookup};
use crate::{DestroyEntityEvent, GameLookup, GameUserLookup, SpawnEntityEvent};
use bevy_ecs::prelude::*;
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_log::{info, warn};
use bevy_transform::prelude::Transform;
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};
use shared::game::defs::{EntityDef, EntityType, TowerRef};
use shared::game::destroyment_method::DestroymentMethod;
use shared::game::owner::Owner;
use shared::game::player::{PlayerName, SharedPlayer};
use shared::game::position::vec3_to_vec2;
use shared::game::shared_game::SharedGame;
use shared::protocol::Protocol;
use shared::Channels;

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        println!("got authorize event");
        if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
            println!("accept!");
            server.accept_connection(user_key);
        }
    }
}

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    for event in event_reader.iter() {
        info!("got connection event");
        let ConnectionEvent(user_key) = event;
        let address = server.user_mut(user_key).address();

        info!("Naia Server connected to: {}", address);
    }
}

pub fn disconnection_event(
    mut event_reader: EventReader<DisconnectionEvent>,
    // mut global: ResMut<Global>,
    mut server: Server<Protocol, Channels>,
    mut player_queue: ResMut<PlayerQueue>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, user) = event;
        info!("Disconnected: {:?}", user.address);
        warn!("TODO: Cleanup");

        // if let Some(entity) = global.user_to_prediction_map.remove(user_key) {
        //     server
        //         .entity_mut(&entity)
        //         .leave_room(&global.main_room_key)
        //         .despawn();
        // }
    }
}

pub fn receive_message_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut player_queue: ResMut<PlayerQueue>,
    mut player_lookup: ResMut<PlayerLookup>,
    game_user_lookup: Res<GameUserLookup>,
    game_lookup: Res<GameLookup>,
    mut server: Server<Protocol, Channels>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,

    // For combos, TODO: move to system
    mut destroy_entity_event: EventWriter<DestroyEntityEvent>,
    mut tower_query: Query<(&Transform, &TowerRef, &Owner)>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(user_key, Channels::PlayerCommand, cmd) = event {
            match cmd {
                Protocol::Auth(_) => {
                    warn!("Client sent auth on already connected.")
                }
                Protocol::JoinRandomGame(join_random_game) => {
                    let name = (*join_random_game.name).clone();
                    let player_name = PlayerName::new(name.as_str());
                    println!("player requesting random game! {:?}", &player_name);

                    let player = SharedPlayer::new_waiting(player_name);
                    player_lookup.0.insert(user_key.clone(), player);

                    player_queue.add(user_key.clone());
                }
                Protocol::JoinFriendGame(_) => {
                    warn!("TODO JoinFriendGame");
                }
                Protocol::GameReady(_) => {
                    warn!("Got a game ready event from client");
                }
                Protocol::NewTowerRequest(place_tower) => {
                    // TODO: Check if possible
                    warn!("Check if building is possible");
                    let position = Some(place_tower.position().into());
                    let player = match player_lookup.0.get(&user_key) {
                        Some(a) => a,
                        None => {
                            warn!("Player not found in lookup");
                            continue;
                        }
                    };
                    let game_id = match game_user_lookup.get_user_game(&user_key) {
                        Some(a) => a.clone(),
                        None => {
                            warn!("Player not found in game_user lookup");
                            continue;
                        }
                    };

                    spawn_entity_events.send(SpawnEntityEvent {
                        game_id,
                        entity_def: EntityDef {
                            entity_type: EntityType::Tower,
                            position,
                            owner: Some(player.owner.clone()),
                            tower: Some("machine".to_string()),
                            ..Default::default()
                        },
                    })

                    // server.send_message(user_key, Channels::ServerCommand, &assignment_message);
                }
                Protocol::ComboTowerRequest(combo_tower_request) => {
                    let player = match player_lookup.0.get(&user_key) {
                        Some(a) => a,
                        None => {
                            warn!("Player not found in lookup");
                            continue;
                        }
                    };
                    let game_id = match game_user_lookup.get_user_game(&user_key) {
                        Some(a) => a.clone(),
                        None => {
                            warn!("Player not found in game_user lookup");
                            continue;
                        }
                    };

                    let game = match game_lookup.0.get(&game_id) {
                        None => {
                            warn!("Game not found in lookup for combo tower request");
                            continue;
                        }
                        Some(s) => s,
                    };

                    let server_ids = &*combo_tower_request.towers;
                    warn!("TODO: Check if building is possible");
                    warn!("TODO: Check which tower this upgrades to!");
                    warn!("TODO: Check owner");

                    let last_tower_id = server_ids.last().unwrap();
                    let last_tower_entity = match game.entities.get(last_tower_id) {
                        None => {
                            warn!("Last tower not found in game entities");
                            continue;
                        }
                        Some(s) => s,
                    };
                    let (transform, tower_ref, owner) = match tower_query.get(*last_tower_entity) {
                        Err(e) => {
                            warn!("Last tower not found in query for combo tower.");
                            continue;
                        }
                        Ok(s) => s,
                    };
                    let position = vec3_to_vec2(&transform.translation);

                    spawn_entity_events.send(SpawnEntityEvent {
                        game_id,
                        entity_def: EntityDef {
                            entity_type: EntityType::Tower,
                            position: Some(position.into()),
                            owner: Some(player.owner.clone()),
                            tower: Some("machine".to_string()),
                            ..Default::default()
                        },
                    });

                    for server_entity_id in server_ids {
                        destroy_entity_event.send(DestroyEntityEvent {
                            game_id,
                            server_entity_id: server_entity_id.clone(),
                            destroyment_method: DestroymentMethod::Quiet,
                        });
                    }
                }
                Protocol::SpawnEntity(_) => {
                    warn!("Got a spawn entity event from client");
                }
                Protocol::ReleaseCreeps(_) => {
                    warn!("Got a release the creeps event from client");
                }
                Protocol::UpdatePosition(_) => {
                    warn!("Got an update position from client");
                }
                Protocol::DestroyEntity(_) => {
                    warn!("Got a destroy entity from client");
                }
                Protocol::UpdatePlayer(_) => {
                    warn!("Got an update player from client");
                }
                Protocol::GameOver(_) => {
                    warn!("Got a game over from client");
                }
            }
            info!(key = ?user_key.to_u64())
        }
    }
}
