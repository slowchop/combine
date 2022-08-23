use crate::app::GameState;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use bevy::prelude::*;
use bevy::utils::HashSet;
use iyes_loopless::prelude::NextState;
use naia_bevy_client::events::{InsertComponentEvent, MessageEvent, UpdateComponentEvent};
use naia_bevy_client::{Client, CommandsExt};
use shared::game::shared_game::SharedGame;
use shared::game::ClientGameInfo;
use shared::protocol::{Protocol, ProtocolKind};
use shared::Channels;

pub fn connect_event(client: Client<Protocol, Channels>) {
    println!("Client connected to: {}", client.server_address());
}

pub fn disconnect_event(client: Client<Protocol, Channels>) {
    println!("Client disconnected from: {}", client.server_address());
}

/// The server always sends two of the same packet to the client.
///
/// I'm sure that the higher level server code isn't sending anything so it seems like naia is
/// doing it. I don't know why, since it should be "ordered reliable".
///
/// This is to track duplicate packets.
#[derive(Default)]
pub struct SeenHack(HashSet<u64>);

impl SeenHack {
    fn seen(&mut self, id: u64) -> bool {
        if self.0.contains(&id) {
            true
        } else {
            self.0.insert(id);
            false
        }
    }
}

pub fn receive_message_event(
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    client: Client<Protocol, Channels>,
    mut seen_hack: ResMut<SeenHack>,
    mut spawn_entity_event: EventWriter<SpawnEntityEvent>,
) {
    // dbg!(client.is_connected());
    for event in event_reader.iter() {
        println!("event");
        if let MessageEvent(Channels::ServerCommand, msg) = event {
            match msg {
                Protocol::SpawnEntity(spawn_entity) => {
                    let spawn_entity = &*spawn_entity.entity_def;
                    dbg!(&spawn_entity);
                    spawn_entity_event.send(SpawnEntityEvent {
                        server_entity_id: None,
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
                Protocol::Position(_) => {
                    println!("C got a position event from the server?")
                }
            }
        }
    }
}
