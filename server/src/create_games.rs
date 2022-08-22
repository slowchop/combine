use crate::state::{GameId, GameLookup, GameUserLookup, PlayerLookup};
use crate::SpawnServerEntityEvent;
use bevy_ecs::prelude::*;
use bevy_log::{error, warn};
use naia_bevy_server::{Server, UserKey};
use shared::game::defs::{Defs, EntityType};
use shared::game::player::{PlayerName, SharedPlayer};
use shared::game::shared_game::SharedGame;
use shared::protocol::game_ready::GameReady;
use shared::protocol::Protocol;
use shared::Channels;

pub struct CreateGameEvent {
    pub user_keys: Vec<UserKey>,
}

pub fn create_games(
    mut server: Server<Protocol, Channels>,
    mut game_user_lookup: ResMut<GameUserLookup>,
    mut game_lookup: ResMut<GameLookup>,
    mut player_lookup: ResMut<PlayerLookup>,
    mut spawn_entities: EventWriter<SpawnServerEntityEvent>,
    mut create_game_events: EventReader<CreateGameEvent>,
    defs: Res<Defs>,
) {
    for create_game_event in create_game_events.iter() {
        let player_names: Vec<PlayerName> = create_game_event
            .user_keys
            .iter()
            .map(|&u| player_lookup.0.get(&u).unwrap().name.clone())
            .collect::<Vec<_>>();

        // Create SharedPlayers.
        let shared_players: Vec<SharedPlayer> = player_names
            .iter()
            .map(|n| SharedPlayer::new(n.clone()))
            .collect::<Vec<_>>();
        let map_name = "test";
        let game = SharedGame::new(map_name.to_string(), shared_players);

        // Create GameId.
        let game_id = game_user_lookup.create_game_reference(create_game_event.user_keys.clone());
        game_lookup.0.insert(game_id, game);

        // Send GameReady to each player.
        for (idx, player) in create_game_event.user_keys.iter().enumerate() {
            println!("Sending GameReady to {}", player_names[idx]);
            let message = GameReady::new(player_names.clone(), idx as u8, map_name.to_string());
            server.send_message(player, Channels::ServerCommand, &message);
        }

        // Get the level and start spawn requesting!
        let level = defs.levels.get(map_name).unwrap();
        for entity_def in &level.entities {
            match entity_def.entity_type {
                // We doesn't care about these.
                EntityType::Sprite => {}
                EntityType::Ground => {}
                EntityType::Base => {}
                EntityType::Path => {}
                EntityType::Creep => {
                    warn!("No creeps should be in a level!");
                }
                _ => {
                    println!("Sending {:?} to spawn_entities", entity_def);
                    spawn_entities.send(SpawnServerEntityEvent {
                        game_id,
                        entity_def: entity_def.clone(),
                    });
                }
            }
        }
    }
}
