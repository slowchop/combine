use crate::state::GameId;
use crate::{GameLookup, GameUserLookup};
use bevy_ecs::prelude::*;
use bevy_log::warn;
use naia_bevy_server::Server;
use shared::game::defs::EntityDef;
use shared::game::shared_game::ServerEntityId;
use shared::protocol::spawn_entity::SpawnEntity;
use shared::protocol::Protocol;
use shared::Channels;

pub struct NewEntityEvent {
    pub game_id: GameId,
    pub entity: Entity,
    pub entity_def: EntityDef,
}

pub fn add_new_entities_to_game(
    mut commands: Commands,
    mut new_entities_events: EventReader<NewEntityEvent>,
    mut game_lookup: ResMut<GameLookup>,
) {
    for new_entity_event in new_entities_events.iter() {
        println!("{:?}", new_entity_event.entity);
        let game_id = new_entity_event.game_id;
        let game = match game_lookup.0.get_mut(&game_id) {
            Some(g) => g,
            None => {
                warn!("Could not get game for game_id {:?}", game_id);
                continue;
            }
        };

        let server_entity_id = game.add_entity(new_entity_event.entity);
        commands
            .entity(new_entity_event.entity)
            .insert(server_entity_id);
    }
}

pub fn send_new_entities_to_players(
    mut new_entities_events: EventReader<NewEntityEvent>,
    game_user_lookup: Res<GameUserLookup>,
    mut server: Server<Protocol, Channels>,
) {
    for new_entity_event in new_entities_events.iter() {
        let users = match game_user_lookup.get_game_players(&new_entity_event.game_id) {
            Some(u) => u,
            None => {
                warn!(
                    "Could not get users for game_id {:?}",
                    new_entity_event.game_id
                );
                continue;
            }
        };

        if users.len() == 0 {
            warn!(
                "Could not get game_user_lookup for game_id {:?}",
                new_entity_event.game_id
            );
            continue;
        }

        let message = SpawnEntity::new(&new_entity_event.entity_def);
        for user in users {
            server.send_message(user, Channels::ServerCommand, &message);
        }
    }
}
