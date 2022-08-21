use crate::app::GameState;
use crate::states::playing::GameInfo;
use bevy::prelude::*;
use iyes_loopless::prelude::NextState;
use naia_bevy_client::events::{
    InsertComponentEvent, MessageEvent, SpawnEntityEvent, UpdateComponentEvent,
};
use naia_bevy_client::{Client, CommandsExt};
use shared::protocol::{Protocol, ProtocolKind};
use shared::Channels;

pub fn connect_event(client: Client<Protocol, Channels>) {
    println!("Client connected to: {}", client.server_address());
}

pub fn disconnect_event(client: Client<Protocol, Channels>) {
    println!("Client disconnected from: {}", client.server_address());
}

pub fn receive_message_event(
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvent<Protocol, Channels>>,
    client: Client<Protocol, Channels>,
) {
    // dbg!(client.is_connected());
    for event in event_reader.iter() {
        println!("event");
        if let MessageEvent(Channels::ServerCommand, msg) = event {
            match msg {
                Protocol::Auth(_) => {}
                Protocol::JoinRandomGame(_) => {}
                Protocol::JoinFriendGame(_) => {}
                Protocol::GameReady(game_ready) => {
                    println!(
                        "Client got a game ready! {} {:?} {}",
                        *game_ready.level, *game_ready.player_names, *game_ready.you_are
                    );
                    let game_info: GameInfo = game_ready.into();
                    commands.spawn().insert(game_info);
                    commands.insert_resource(NextState(GameState::LoadingLevel));
                }
                Protocol::RequestTowerPlacement(_) => {
                    todo!("place tower")
                }
            }
        }
    }
}

pub fn spawn_entity_event(mut event_reader: EventReader<SpawnEntityEvent>) {
    for event in event_reader.iter() {
        match event {
            SpawnEntityEvent(_entity) => {
                info!("spawned entity");
            }
        }
    }
}

pub fn insert_component_event(
    mut event_reader: EventReader<InsertComponentEvent<ProtocolKind>>,
    mut local: Commands,
    // color_query: Query<&Color>,
) {
    for event in event_reader.iter() {
        if let InsertComponentEvent(entity, protocol_kind) = event {
            println!("insert component event {:?}", protocol_kind);

            // if let Ok(color) = color_query.get(*entity) {
            //     info!("add color to entity");
            //
            //     let color = {
            //         match *color.value {
            //             ColorValue::Red => BevyColor::RED,
            //             ColorValue::Blue => BevyColor::BLUE,
            //             ColorValue::Yellow => BevyColor::YELLOW,
            //         }
            //     };
            //
            //     local.entity(*entity).insert_bundle(SpriteBundle {
            //         sprite: Sprite {
            //             custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
            //             color,
            //             ..Default::default()
            //         },
            //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
            //         ..Default::default()
            //     });
            // }
        }
    }
}

pub fn update_component_event(
    mut event_reader: EventReader<UpdateComponentEvent<ProtocolKind>>,
    // mut position_query: Query<&mut Position>,
) {
    for event in event_reader.iter() {
        // dbg!(event);
        println!("got update component event");
    }

    // if let Some(owned_entity) = &global.owned_entity {
    //     let mut latest_tick: Option<Tick> = None;
    //     let server_entity = owned_entity.confirmed;
    //     let client_entity = owned_entity.predicted;
    //
    //     for event in event_reader.iter() {
    //         let UpdateComponentEvent(server_tick, updated_entity, _) = event;
    //
    //         // If entity is owned
    //         if *updated_entity == server_entity {
    //             if let Some(last_tick) = &mut latest_tick {
    //                 if sequence_greater_than(*server_tick, *last_tick) {
    //                     *last_tick = *server_tick;
    //                 }
    //             } else {
    //                 latest_tick = Some(*server_tick);
    //             }
    //         }
    //     }
    //
    //     if let Some(server_tick) = latest_tick {
    //         if let Ok([server_position, mut client_position]) =
    //             position_query.get_many_mut([server_entity, client_entity])
    //         {
    //             let replay_commands = global.command_history.replays(&server_tick);
    //
    //             // set to authoritative state
    //             client_position.x.mirror(&server_position.x);
    //             client_position.y.mirror(&server_position.y);
    //
    //             // Replay all stored commands
    //             for (_command_tick, command) in replay_commands {
    //                 shared_behavior::process_command(&command, &mut client_position);
    //             }
    //         }
    //     }
    // }
}
