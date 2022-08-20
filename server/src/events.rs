use crate::state::State;
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_log::info;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    shared::Random,
    Server,
};
use shared::{Channels, Protocol};

// use crate::resources::Global;

pub fn authorization_event(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        println!("got authorize event");
        // if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
        //     if *auth.username == "charlie" && *auth.password == "12345" {
        //         // Accept incoming connection
        //         server.accept_connection(user_key);
        //     } else {
        //         // Reject incoming connection
        //         server.reject_connection(user_key);
        //     }
        // }
    }
}

pub fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut state: ResMut<State>,
    mut server: Server<'world, 'state, Protocol, Channels>,
) {
    for event in event_reader.iter() {
        println!("got connection event");
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
    // mut global: ResMut<Global>,
    server: Server<Protocol, Channels>,
) {
    for event in event_reader.iter() {
        println!("got message event");
        // if let MessageEvent(_user_key, Channels::PlayerCommand, Protocol::KeyCommand(key_command)) =
        //     event
        // {
        //     if let Some(entity) = &key_command.entity.get(&server) {
        //         // global
        //         //     .player_last_command
        //         //     .insert(*entity, key_command.clone());
        //     }
        // }
    }
}
