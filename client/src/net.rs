use crate::app::GameState;
use crate::states::playing::console::ConsoleItem;
use crate::states::playing::hurt_entities::HurtEntityEvent;
use crate::states::playing::spawn_entities::{HasColdEffect, SpawnEntityEvent};
use crate::states::playing::update_player::UpdatePlayerEvent;
use bevy::prelude::*;
use bevy::utils::HashSet;
use iyes_loopless::prelude::NextState;
use naia_bevy_client::events::{InsertComponentEvent, MessageEvent, UpdateComponentEvent};
use naia_bevy_client::{Client, CommandsExt};
use shared::game::destroyment_method::DestroymentMethod;
use shared::game::owner::Owner;
use shared::game::shared_game::{ServerEntityId, SharedGame};
use shared::game::ClientGameInfo;
use shared::protocol::release_creep::ReleaseCreeps;
use shared::protocol::{Protocol, ProtocolKind};
use shared::ticks::Ticks;
use shared::Channels;
use std::thread::spawn;
use std::time::Duration;

pub fn connect_event(client: Client<Protocol, Channels>) {
    println!("Client connected {}", client.server_address());
}

pub fn disconnect_event(mut commands: Commands, game: Query<&SharedGame>) {
    let game = if let Ok(game) = game.get_single() {
        game
    } else {
        println!("Client disconnected, but we have no game.");
        return;
    };
    if game.winner.is_some() {
        println!("Client disconnected, but the game is over.");
        return;
    }

    println!("Client disconnected and we're in a game! Show the disconnect screen.");
    commands.insert_resource(NextState(GameState::Disconnected));
}

#[derive(Debug)]
pub struct ReleaseCreepEvent {
    pub server_entity_id: ServerEntityId,
}

#[derive(Debug)]
pub struct UpdatePositionEvent {
    pub server_entity_id: ServerEntityId,
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Debug)]
pub struct DestroyEntityEvent {
    pub server_entity_id: ServerEntityId,
    pub how: DestroymentMethod,
}

#[derive(Debug)]
pub struct GameOverEvent {
    pub winner: Owner,
}

pub fn receive_message_event(
    time: Res<Time>,
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut spawn_entity_event: EventWriter<SpawnEntityEvent>,
    mut release_the_creeps_events: EventWriter<ReleaseCreepEvent>,
    mut update_position_events: EventWriter<UpdatePositionEvent>,
    mut destroy_entity_events: EventWriter<DestroyEntityEvent>,
    mut update_player_events: EventWriter<UpdatePlayerEvent>,
    mut game_over_events: EventWriter<GameOverEvent>,
    mut hurt_entity_events: EventWriter<HurtEntityEvent>,
    mut console: EventWriter<ConsoleItem>,
    client_game_info: Query<&ClientGameInfo>,
    game: Query<&SharedGame>,
    mut cold_query: Query<&mut HasColdEffect>,
) {
    // dbg!(client.is_connected());
    for event in event_reader.iter() {
        if let MessageEvent(Channels::ServerCommand, msg) = event {
            match msg {
                Protocol::SpawnEntity(spawn_entity) => {
                    let spawn_entity = &*spawn_entity.entity_def;
                    if spawn_entity.server_entity_id.is_none() {
                        warn!(
                            "Got a spawn entity message without a server entity id {:?}",
                            spawn_entity
                        );
                    }
                    spawn_entity_event.send(SpawnEntityEvent {
                        server_entity_id: spawn_entity.server_entity_id,
                        entity_def: spawn_entity.clone(),
                    });
                }
                Protocol::Auth(_) => {}
                Protocol::JoinRandomGame(_) => {}
                Protocol::JoinFriendGame(_) => {}
                Protocol::GameReady(game_ready) => {
                    let client_game_info = &*game_ready.game_info;
                    println!("-------- Client got a game ready! {:?}", client_game_info);

                    commands.spawn().insert(client_game_info.clone());

                    let shared_game = SharedGame::new(
                        client_game_info.map.clone(),
                        client_game_info.players.clone(),
                    );
                    commands.spawn().insert(shared_game);

                    commands.insert_resource(NextState(GameState::LoadingLevel));
                }
                Protocol::NewTowerRequest(_) => {
                    warn!("Got a request tower placement message, but we are not a server");
                }
                Protocol::ReleaseCreeps(release_creeps_event) => {
                    info!("got a release the creeps network message.");
                    for server_entity_id in &*release_creeps_event.creeps {
                        release_the_creeps_events.send(ReleaseCreepEvent {
                            server_entity_id: *server_entity_id,
                        });
                    }
                }
                Protocol::UpdatePosition(update_position) => {
                    update_position_events.send(UpdatePositionEvent {
                        position: (*update_position.position).clone().into(),
                        server_entity_id: (*update_position.server_entity_id),
                        velocity: (*update_position.velocity).clone().into(),
                    });
                }
                Protocol::DestroyEntity(destroy_entity) => {
                    destroy_entity_events.send(DestroyEntityEvent {
                        server_entity_id: (*destroy_entity.server_entity_id),
                        how: (*destroy_entity.how),
                    });

                    if let Some(earned_for) = *destroy_entity.gold_earned_for {
                        let client_game_info = client_game_info.get_single().unwrap();
                        if earned_for == client_game_info.i_am {
                            console.send(ConsoleItem::new(format!(
                                "You earned {} gold for killing a creep!",
                                *destroy_entity.gold_earned
                            )));
                        }
                    }
                }
                Protocol::UpdatePlayer(update_player) => {
                    update_player_events.send(UpdatePlayerEvent {
                        owner: (*update_player.owner),
                        gold: (*update_player.gold),
                        lives: (*update_player.lives),
                    });
                }
                Protocol::GameOver(game_over) => {
                    info!("Got a game over message");
                    game_over_events.send(GameOverEvent {
                        winner: *game_over.winner,
                    });
                }
                Protocol::ComboTowerRequest(_) => {
                    warn!("Got a combo tower request message");
                }
                Protocol::ComboCreepRequest(_) => {
                    warn!("Got a combo creep request message");
                }
                Protocol::HurtEntity(hurt_entity) => {
                    let src = *hurt_entity.src_server_entity_id;
                    let dst = *hurt_entity.dst_server_entity_id;
                    let total_damaged = *hurt_entity.total_damaged;

                    hurt_entity_events.send(HurtEntityEvent {
                        src,
                        dst,
                        total_damaged,
                    });
                }
                Protocol::ServerMessage(msg) => {
                    let s = &*msg.text;
                    console.send(ConsoleItem::new(s.to_string()));
                }
                Protocol::HotCreep(_) => {
                    warn!("Got a hot creep message");
                }
                Protocol::ColdCreep(cold) => {
                    let server_entity_id = *cold.server_entity_id;
                    let game = game.get_single().unwrap();
                    // let entity = if let Ok(e) = &game.entities.get(&server_entity_id) {
                    //     e
                    // } else {
                    //     continue;
                    // };
                    //
                    // let mut cold_effect = cold_query.get_mut(*entity).unwrap();
                    // cold_effect.until =
                    //     time.time_since_startup() + Duration::from_secs_f32(*cold.duration);
                    warn!("TODO Protocol::ColdCreep in net.rs");
                }
            }
        }
    }
}
