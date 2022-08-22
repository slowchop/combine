use crate::state::PlayerLookup;
use crate::state::PlayerQueue;
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_log::{info, warn};
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};
use shared::game::player::{PlayerName, SharedPlayer};
use shared::protocol::position::Position;
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
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        println!("got message event");
        if let MessageEvent(user_key, Channels::PlayerCommand, cmd) = event {
            match cmd {
                Protocol::Auth(_) => {
                    warn!("Auth on already connected?")
                }
                Protocol::JoinRandomGame(join_random_game) => {
                    let name = (*join_random_game.name).clone();
                    let player_name = PlayerName::new(name.as_str());
                    println!("player requesting random game! {:?}", &player_name);

                    let player = SharedPlayer {
                        name: player_name,
                        gold: 0,
                        lives: 0,
                    };
                    player_lookup.0.insert(user_key.clone(), player);

                    player_queue.add(user_key.clone());
                }
                Protocol::JoinFriendGame(_) => {
                    todo!();
                }
                Protocol::GameReady(_) => {
                    // Server message. Ignored.
                    panic!();
                }
                Protocol::RequestTowerPlacement(place_tower) => {
                    println!("REQQQQQQQQ");
                    // TODO: Check if possible
                    let position = Position::new(place_tower.position());
                    let server_player = &player_lookup.0[&user_key];
                    todo!()

                    // server.send_message(user_key, Channels::ServerCommand, &assignment_message);
                }
                Protocol::Position(_) => {
                    panic!();
                    println!("S got a position event from client")
                }
            }
            info!(key = ?user_key.to_u64())
            //     if let Some(entity) = &key_command.entity.get(&server) {
            //         // global
            //         //     .player_last_command
            //         //     .insert(*entity, key_command.clone());
            //     }
        }
    }
}
