use crate::state::PlayerQueue;
use crate::state::{GameId, PlayerLookup};
use crate::{GameLookup, GameUserLookup, SpawnEntityEvent};
use bevy_ecs::prelude::{EventWriter, Res};
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_log::{info, warn};
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Server,
};
use shared::game::defs::{EntityDef, EntityType};
use shared::game::owner::Owner;
use shared::game::player::{PlayerName, SharedPlayer};
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
) {
    for event in event_reader.iter() {
        info!("Got message event!");
        if let MessageEvent(user_key, Channels::PlayerCommand, cmd) = event {
            match cmd {
                Protocol::Auth(_) => {
                    warn!("Client sent auth on already connected.")
                }
                Protocol::JoinRandomGame(join_random_game) => {
                    let name = (*join_random_game.name).clone();
                    let player_name = PlayerName::new(name.as_str());
                    println!("player requesting random game! {:?}", &player_name);

                    let player = SharedPlayer {
                        name: player_name,
                        gold: 0,
                        lives: 0,
                        owner: Owner::waiting(),
                    };
                    player_lookup.0.insert(user_key.clone(), player);

                    player_queue.add(user_key.clone());
                }
                Protocol::JoinFriendGame(_) => {
                    warn!("TODO JoinFriendGame");
                }
                Protocol::GameReady(_) => {
                    warn!("Got a game ready event from client");
                }
                Protocol::RequestTowerPlacement(place_tower) => {
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
                Protocol::NetPosition(_) => {
                    warn!("Got a position event from client");
                }
                Protocol::SpawnEntity(_) => {
                    warn!("Got a spawn entity event from client");
                }
                Protocol::ReleaseCreep(_) => {
                    warn!("Got a release the creeps event from client");
                }
            }
            info!(key = ?user_key.to_u64())
        }
    }
}
