use crate::app::GameState;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use crate::states::playing::GameInfo;
use bevy::prelude::*;
use bevy::utils::HashSet;
use iyes_loopless::prelude::NextState;
use naia_bevy_client::events::{InsertComponentEvent, MessageEvent, UpdateComponentEvent};
use naia_bevy_client::{Client, CommandsExt};
use shared::game::shared_game::SharedGame;
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
                    let spawn_entity = match spawn_entity.to_entity_def() {
                        Ok(s) => s,
                        Err(e) => {
                            error!("Error decoding spawn entity: {:?}", e);
                            continue;
                        }
                    };
                    dbg!(&spawn_entity);
                    spawn_entity_event.send(SpawnEntityEvent {
                        server_entity_id: None,
                        entity_def: spawn_entity,
                    });
                }
                Protocol::Auth(_) => {}
                Protocol::JoinRandomGame(_) => {}
                Protocol::JoinFriendGame(_) => {}
                Protocol::GameReady(game_ready) => {
                    println!(
                        "-------- Client got a game ready! {} {:?} {}",
                        *game_ready.map, *game_ready.player_names, *game_ready.i_am
                    );

                    // let game_info: GameInfo = game_ready.into();
                    // commands.spawn().insert(game_info);

                    let shared_game =
                        SharedGame::new((*game_ready.map).clone(), game_ready.player_names());

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
