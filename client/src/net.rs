use crate::app::GameState;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use bevy::prelude::*;
use bevy::utils::HashSet;
use iyes_loopless::prelude::NextState;
use naia_bevy_client::events::{InsertComponentEvent, MessageEvent, UpdateComponentEvent};
use naia_bevy_client::{Client, CommandsExt};
use shared::game::destroyment_method::DestroymentMethod;
use shared::game::shared_game::{ServerEntityId, SharedGame};
use shared::game::ClientGameInfo;
use shared::protocol::release_creep::ReleaseCreeps;
use shared::protocol::{Protocol, ProtocolKind};
use shared::ticks::Ticks;
use shared::Channels;
use std::thread::spawn;

pub fn connect_event(client: Client<Protocol, Channels>) {
    println!("Client connected to: {}", client.server_address());
}

pub fn disconnect_event(client: Client<Protocol, Channels>) {
    println!("Client disconnected from: {}", client.server_address());
}

#[derive(Debug)]
pub struct ReleaseCreepEvent {
    // pub server_entity_id: ServerEntityId,
    // pub starting_position: Vec3,
    // pub starting_tick: Ticks,
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
    pub position: Vec3,
    pub velocity: Vec3,
    pub how: DestroymentMethod,
}

pub fn receive_message_event(
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    mut spawn_entity_event: EventWriter<SpawnEntityEvent>,
    mut release_the_creeps_events: EventWriter<ReleaseCreepEvent>,
    mut update_position_events: EventWriter<UpdatePositionEvent>,
    mut destroy_entity_events: EventWriter<DestroyEntityEvent>,
) {
    // dbg!(client.is_connected());
    for event in event_reader.iter() {
        println!("event");
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
                Protocol::RequestTowerPlacement(_) => {
                    todo!("place tower")
                }
                Protocol::ReleaseCreeps(_) => {
                    info!("got a release the creeps network message.");
                    release_the_creeps_events.send(ReleaseCreepEvent {});
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
                        position: (*destroy_entity.position).clone().into(),
                        server_entity_id: (*destroy_entity.server_entity_id),
                        velocity: (*destroy_entity.velocity).clone().into(),
                        how: (*destroy_entity.how),
                    });
                }
            }
        }
    }
}
