use crate::release_creeps::send_message_to_game;
use crate::state::GameId;
use crate::state::PlayerQueue;
use crate::{
    DestroyEntityEvent, GameLookup, GamePlayerHasDisconnected, GameUserLookup, SpawnEntityEvent,
};
use bevy_ecs::prelude::*;
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_log::{info, warn};
use bevy_transform::prelude::Transform;
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};
use shared::game::defs::{CreepRef, Defs, EntityDef, EntityType, TowerRef};
use shared::game::destroyment_method::DestroymentMethod;
use shared::game::owner::Owner;
use shared::game::player::{PlayerName, SharedPlayer};
use shared::game::position::vec3_to_vec2;
use shared::game::shared_game::{Multiplier, SharedGame};
use shared::protocol::server_message::ServerMessage;
use shared::protocol::update_player::UpdatePlayer;
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
    mut commands: Commands,
    mut event_reader: EventReader<DisconnectionEvent>,
    mut game_player_has_disconnected: ResMut<GamePlayerHasDisconnected>,
    mut game_user_lookup: ResMut<GameUserLookup>,
    mut game_lookup: ResMut<GameLookup>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, user) = event;
        info!("Disconnected: {:?}", user.address);

        let (game_id, _) = if let Some(g) = game_user_lookup.get_user_game_and_owner(user_key) {
            g.clone()
        } else {
            warn!(
                "Could not find game for disconnected user {:?}",
                user.address
            );
            continue;
        };

        let previous_player_has_disconnected = game_player_has_disconnected.0.contains(&game_id);
        if previous_player_has_disconnected {
            info!("Cleaning up game: {:?}", game_id);

            // game_player_has_disconnected
            game_player_has_disconnected.0.remove(&game_id);

            // game_user_lookup
            let v = vec![];
            let existing_user_keys = game_user_lookup.get_game_users(&game_id).unwrap_or(&v);

            for existing_user_key in existing_user_keys.clone() {
                game_user_lookup.user_to_game.remove(&existing_user_key);
            }
            game_user_lookup.game_to_users.remove(&game_id);

            // game_lookup
            if let Some(game) = game_lookup.0.get(&game_id) {
                let mut count = 0;
                for (_, entity) in &game.entities {
                    commands.entity(*entity).despawn();
                    count += 1;
                }
                info!("Despawned {} entities", count);
            }
            game_lookup.0.remove(&game_id);
        } else {
            game_player_has_disconnected.0.insert(game_id);
        }
    }
}

pub fn receive_message_event(
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut player_queue: ResMut<PlayerQueue>,
    game_user_lookup: Res<GameUserLookup>,
    mut game_lookup: ResMut<GameLookup>,
    mut server: Server<Protocol, Channels>,
    mut spawn_entity_events: EventWriter<SpawnEntityEvent>,

    // For combos, TODO: move to system
    mut destroy_entity_event: EventWriter<DestroyEntityEvent>,
    mut tower_query: Query<(&Transform, &TowerRef, &Owner)>,
    mut creep_query: Query<(&Transform, &CreepRef, &Owner)>,
    defs: Res<Defs>,
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
                    info!("player requesting random game! {:?}", &player_name);
                    player_queue.add(user_key.clone(), player_name);
                }
                Protocol::JoinFriendGame(_) => {
                    warn!("TODO JoinFriendGame");
                }
                Protocol::GameReady(_) => {
                    warn!("Got a game ready event from client");
                }
                Protocol::NewTowerRequest(place_tower) => {
                    // TODO: Check if possible
                    // warn!("Check if building is possible");
                    let position = Some(place_tower.position().into());
                    let (game_id, owner) = match game_user_lookup.get_user_game_and_owner(&user_key)
                    {
                        Some(a) => a,
                        None => {
                            warn!("Player not found in lookup");
                            continue;
                        }
                    };
                    // let game_id = match game_user_lookup.get_user_game(&user_key) {
                    //     Some(a) => a.clone(),
                    //     None => {
                    //         warn!("Player not found in game_user lookup");
                    //         continue;
                    //     }
                    // };

                    let mut game = match game_lookup.0.get_mut(&game_id) {
                        Some(a) => a,
                        None => {
                            warn!("Game not found in game lookup");
                            continue;
                        }
                    };

                    let mut player = game.get_player_mut(*owner).unwrap();

                    let tower = defs.tower(&TowerRef("machine".into())).unwrap();

                    // Check if the player has enough $.
                    let cost = tower.cost;
                    if player.gold < cost {
                        let message = ServerMessage::new(format!(
                            "Sorry, you don't have enough $ to buy a {}.",
                            tower.title
                        ));
                        server.send_message(user_key, Channels::ServerCommand, &message);
                        continue;
                    }
                    player.gold -= cost;
                    let message = UpdatePlayer::new(player.owner, player.gold, player.lives);
                    send_message_to_game(
                        &mut server,
                        &*game_user_lookup,
                        &game_id,
                        Channels::ServerCommand,
                        &message,
                    );

                    spawn_entity_events.send(SpawnEntityEvent {
                        game_id: *game_id,
                        entity_def: EntityDef {
                            entity_type: EntityType::Tower,
                            position,
                            owner: Some(player.owner.clone()),
                            tower: Some(TowerRef("machine".to_string())),
                            ..Default::default()
                        },
                        speed_multiplier: 1.0,
                        health_multiplier: 1.0,
                    })

                    // server.send_message(user_key, Channels::ServerCommand, &assignment_message);
                }
                Protocol::ComboTowerRequest(combo_tower_request) => {
                    let (game_id, player_owner) =
                        match game_user_lookup.get_user_game_and_owner(&user_key) {
                            Some(a) => a,
                            None => {
                                warn!("Player not found in lookup");
                                continue;
                            }
                        };

                    let game = match game_lookup.0.get_mut(&game_id) {
                        None => {
                            warn!("Game not found in lookup for combo tower request");
                            continue;
                        }
                        Some(s) => s,
                    };

                    let mut tower_refs = Vec::new();
                    for tower_server_id in &*combo_tower_request.towers {
                        let tower_entity = match game.entities.get(tower_server_id) {
                            Some(t) => t,
                            None => {
                                warn!("Tower not found in game entities");
                                continue;
                            }
                        };

                        let (_, tower_ref, tower_owner) = match tower_query.get(*tower_entity) {
                            Ok(t) => t,
                            Err(e) => {
                                warn!("Tower not found in query");
                                continue;
                            }
                        };

                        if tower_owner != player_owner {
                            warn!("Tower owner does not match player");
                            continue;
                        }

                        tower_refs.push(tower_ref.clone());
                    }

                    let server_ids = &*combo_tower_request.towers;

                    let towers = tower_refs.iter().map(|tr| tr).collect::<Vec<&TowerRef>>();
                    let tower = match defs.tower_for_combo(towers.as_slice()) {
                        None => {
                            warn!("No match for combo {:?}", &tower_refs);
                            continue;
                        }
                        Some(c) => c,
                    };

                    let last_tower_id = server_ids.last().unwrap();
                    let last_tower_entity = match game.entities.get(last_tower_id) {
                        None => {
                            warn!("Last tower not found in game entities");
                            continue;
                        }
                        Some(s) => s,
                    };
                    let (transform, _, _) = match tower_query.get(*last_tower_entity) {
                        Err(e) => {
                            warn!("Last tower not found in query for combo tower.");
                            continue;
                        }
                        Ok(s) => s,
                    };
                    let position = vec3_to_vec2(&transform.translation);

                    // Check if the player has enough $.
                    let cost = tower.cost;
                    let player = game.get_player_mut(*player_owner).unwrap();
                    if player.gold < cost {
                        let message = ServerMessage::new(format!(
                            "Sorry, you don't have enough $ to buy a {}.",
                            tower.title
                        ));
                        server.send_message(user_key, Channels::ServerCommand, &message);
                        continue;
                    }
                    player.gold -= cost;
                    let message = UpdatePlayer::new(player.owner, player.gold, player.lives);
                    send_message_to_game(
                        &mut server,
                        &*game_user_lookup,
                        &game_id,
                        Channels::ServerCommand,
                        &message,
                    );

                    info!(?tower.name, "Creating tower");

                    spawn_entity_events.send(SpawnEntityEvent {
                        game_id: *game_id,
                        entity_def: EntityDef {
                            entity_type: EntityType::Tower,
                            position: Some(position.into()),
                            owner: Some(player.owner.clone()),
                            tower: Some(tower.name),
                            ..Default::default()
                        },
                        speed_multiplier: 1.0,
                        health_multiplier: 1.0,
                    });

                    for server_entity_id in server_ids {
                        destroy_entity_event.send(DestroyEntityEvent {
                            game_id: *game_id,
                            server_entity_id: server_entity_id.clone(),
                            destroyment_method: DestroymentMethod::Quiet,
                            gold_earned: 0,
                            gold_earned_for: None,
                        });
                    }
                }
                Protocol::ComboCreepRequest(combo_creep_request) => {
                    let (game_id, player_owner) =
                        match game_user_lookup.get_user_game_and_owner(&user_key) {
                            Some(a) => a,
                            None => {
                                warn!("Player not found in lookup");
                                continue;
                            }
                        };

                    let game = match game_lookup.0.get_mut(&game_id) {
                        None => {
                            warn!("Game not found in lookup for combo creep request");
                            continue;
                        }
                        Some(s) => s,
                    };
                    let Multiplier {
                        health: health_multiplier,
                        speed: speed_multiplier,
                    } = game.multipliers();

                    let server_ids = &*combo_creep_request.creeps;

                    let mut creep_refs = Vec::new();
                    for creep_server_id in &*combo_creep_request.creeps {
                        let creep_entity = match game.entities.get(creep_server_id) {
                            Some(t) => t,
                            None => {
                                warn!("Creep not found in game entities");
                                continue;
                            }
                        };

                        let (_, creep_ref, creep_owner) = match creep_query.get(*creep_entity) {
                            Ok(t) => t,
                            Err(e) => {
                                warn!("Creep not found in query");
                                continue;
                            }
                        };

                        if creep_owner != player_owner {
                            warn!("Creep owner does not match player");
                            continue;
                        }

                        creep_refs.push(creep_ref.clone());
                    }

                    let creeps = creep_refs.iter().map(|tr| tr).collect::<Vec<&CreepRef>>();
                    let creep = match defs.creep_for_combo(creeps.as_slice()) {
                        None => {
                            warn!("No match for combo {:?}", &creeps);
                            continue;
                        }
                        Some(c) => c,
                    };

                    let last_creep_id = server_ids.last().unwrap();
                    let last_creep_entity = match game.entities.get(last_creep_id) {
                        None => {
                            warn!("Last creep not found in game entities");
                            continue;
                        }
                        Some(s) => s,
                    };
                    let (transform, _, _) = match creep_query.get(*last_creep_entity) {
                        Err(e) => {
                            warn!("Last creep not found in query for combo creep.");
                            continue;
                        }
                        Ok(s) => s,
                    };
                    let position = vec3_to_vec2(&transform.translation);

                    let cost = creep.cost;
                    let player = game.get_player_mut(*player_owner).unwrap();
                    if player.gold < cost {
                        let message = ServerMessage::new(format!(
                            "Sorry, you don't have enough $ to buy a {}.",
                            creep.title
                        ));
                        server.send_message(user_key, Channels::ServerCommand, &message);
                        continue;
                    }
                    player.gold -= cost;
                    let message = UpdatePlayer::new(player.owner, player.gold, player.lives);
                    send_message_to_game(
                        &mut server,
                        &*game_user_lookup,
                        &game_id,
                        Channels::ServerCommand,
                        &message,
                    );

                    spawn_entity_events.send(SpawnEntityEvent {
                        game_id: *game_id,
                        entity_def: EntityDef {
                            entity_type: EntityType::Creep,
                            position: Some(position.into()),
                            owner: Some(player.owner.clone()),
                            creep: Some(creep.name),
                            ..Default::default()
                        },
                        speed_multiplier,
                        health_multiplier,
                    });

                    for server_entity_id in server_ids {
                        destroy_entity_event.send(DestroyEntityEvent {
                            game_id: *game_id,
                            server_entity_id: server_entity_id.clone(),
                            destroyment_method: DestroymentMethod::Quiet,
                            gold_earned: 0,
                            gold_earned_for: None,
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
                Protocol::HurtEntity(hurt_entity) => {
                    warn!("Got a hurt entity from client");
                }
                Protocol::ServerMessage(_) => {
                    warn!("Got a hurt entity from client");
                }
                Protocol::HotCreep(_) => {
                    warn!("Got a hot creep from client");
                }
                Protocol::ColdCreep(_) => {
                    warn!("Got a cold creep from client");
                }
            }
        }
    }
}
