use crate::state::{GameId, GameLookup, GameUserLookup};
use crate::SpawnEntityEvent;
use bevy_ecs::prelude::*;
use bevy_log::{error, info, warn};
use naia_bevy_server::{Server, UserKey};
use shared::game::defs::{Defs, EntityType};
use shared::game::owner::Owner;
use shared::game::player::{PlayerName, SharedPlayer};
use shared::game::shared_game::SharedGame;
use shared::game::ClientGameInfo;
use shared::protocol::game_ready::GameReady;
use shared::protocol::Protocol;
use shared::Channels;

pub struct CreateGameEvent {
    pub user_keys: Vec<(UserKey, PlayerName)>,
}

pub fn create_games(
    mut server: Server<Protocol, Channels>,
    mut game_user_lookup: ResMut<GameUserLookup>,
    mut game_lookup: ResMut<GameLookup>,
    mut spawn_entities: EventWriter<SpawnEntityEvent>,
    mut create_game_events: EventReader<CreateGameEvent>,
    defs: Res<Defs>,
) {
    for create_game_event in create_game_events.iter() {
        let shared_players: Vec<SharedPlayer> = create_game_event
            .user_keys
            .iter()
            .enumerate()
            .map(|(idx, (user_key, player_name))| {
                SharedPlayer::new(player_name.clone(), Owner::new(idx as u8))
            })
            .collect::<Vec<_>>();

        let map_name = "jam";
        let game = SharedGame::new(
            map_name.to_string(),
            shared_players.clone(),
            // shared_players.iter().map(|p| p.clone()).collect(),
        );

        // Create GameId.
        let user_keys_and_owners = create_game_event
            .user_keys
            .iter()
            .enumerate()
            .map(|(idx, (user_key, player_name))| (user_key.clone(), Owner::new(idx as u8)))
            .collect::<Vec<(UserKey, Owner)>>()
            .clone();

        let game_id = game_user_lookup.create_game_reference(user_keys_and_owners);
        info!(?game_id, ?game, "Creating game");
        game_lookup.0.insert(game_id, game);

        // Send GameReady to each player.
        for (idx, (user_key, player_name)) in create_game_event.user_keys.iter().enumerate() {
            println!("Sending GameReady to {:?}", shared_players[idx]);
            let client_game_info = ClientGameInfo {
                i_am: Owner::new(idx as u8),
                map: map_name.to_string(),
                players: shared_players.clone(),
            };
            let message = GameReady::new(client_game_info);
            server.send_message(user_key, Channels::ServerCommand, &message);
        }

        // Get the level and start spawn requesting!
        let level = defs.levels.get(map_name).unwrap();
        for entity_def in &level.entities {
            match entity_def.entity_type {
                // We doesn't care about these.
                EntityType::Sprite => {}
                EntityType::Ground => {}
                EntityType::Base => {}
                EntityType::Creep => {
                    warn!("No creeps should be in a level!");
                }
                _ => {
                    spawn_entities.send(SpawnEntityEvent {
                        game_id,
                        entity_def: entity_def.clone(),
                    });
                }
            }
        }
    }
}
