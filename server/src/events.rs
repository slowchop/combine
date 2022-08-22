use crate::server_player::ServerPlayer;
use crate::state::{Global, PlayerQueue, Players};
use bevy_ecs::prelude::Res;
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_log::{info, warn};
use bevy_math::Vec2;
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    shared::Random,
    Server,
};
use shared::game::player_name::PlayerName;
use shared::protocol::entity_assignment::EntityAssignment;
use shared::protocol::game_ready::GameReady;
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
    mut state: ResMut<Global>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    for event in event_reader.iter() {
        info!("got connection event");
        let ConnectionEvent(user_key) = event;
        let address = server
            .user_mut(user_key)
            // Add User to the main Room
            .enter_room(&state.main_room_key)
            // Get User's address for logging
            .address();

        info!("Naia Server connected to: {}", address);

        // global.user_to_prediction_map.insert(*user_key, entity);

        // Send an Entity Assignment message to the User that owns the Square
        // let mut assignment_message = EntityAssignment::new(true);
        // assignment_message.entity.set(&server, &entity);

        // server.send_message(user_key, Channels::EntityAssignment, &assignment_message);
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
    mut players: ResMut<Players>,
    mut server: Server<Protocol, Channels>,
    mut global: Res<Global>,
) {
    for key in server.user_keys() {
        let user = server.user(&key);
    }

    for event in event_reader.iter() {
        println!("got message event");
        if let MessageEvent(user_key, Channels::PlayerCommand, cmd) = event {
            match cmd {
                Protocol::Auth(_) => {
                    warn!("Auth on already connected?")
                }
                Protocol::JoinRandomGame(random_game) => {
                    let name = (*random_game.name).clone();
                    let player_name = PlayerName::new(name.as_str());
                    let player = ServerPlayer {
                        name: player_name,
                        room: global.main_room_key,
                    };
                    println!("player requesting random game! {:?}", &player.name);
                    players.0.insert(user_key.clone(), player);
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

                    let server_player = &players.0[&user_key];
                    dbg!(server_player.room.to_u64());

                    let entity = server
                        // Spawn new Square Entity
                        .spawn()
                        // Add Entity to main Room
                        .enter_room(&server_player.room)
                        // Insert Position component
                        .insert(position)
                        // Insert Color component
                        // .insert(color)
                        // return Entity id
                        .id();

                    println!("sending event");

                    let mut assignment_message = EntityAssignment::new(true);
                    assignment_message.entity.set(&server, &entity);
                    server.send_message(user_key, Channels::ServerCommand, &assignment_message);
                }
                Protocol::EntityAssignment(_) => {
                    panic!();
                    // Server message.
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
