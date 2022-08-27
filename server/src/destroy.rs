use crate::release_creeps::send_message_to_game;
use crate::state::GameId;
use crate::{GameLookup, GameUserLookup};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use naia_bevy_server::Server;
use shared::game::destroyment_method::DestroymentMethod;
use shared::game::shared_game::ServerEntityId;
use shared::protocol::destroy_entity::DestroyEntity;
use shared::protocol::Protocol;
use shared::Channels;

pub struct DestroyEntityEvent {
    pub game_id: GameId,
    pub server_entity_id: ServerEntityId,
    pub destroyment_method: DestroymentMethod,
}

pub fn destroy_entities(
    mut commands: Commands,
    mut destroy_entity_event: EventReader<DestroyEntityEvent>,
    mut game_lookup: ResMut<GameLookup>,
    mut server: Server<Protocol, Channels>,
    game_user_lookup: Res<GameUserLookup>,
) {
    for destroy_entity_event in destroy_entity_event.iter() {
        let game = if let Some(game) = game_lookup.0.get_mut(&destroy_entity_event.game_id) {
            game
        } else {
            warn!("No game when destroying entities");
            return;
        };

        let entity = if let Some(e) = game.entities.get(&destroy_entity_event.server_entity_id) {
            e
        } else {
            warn!(
                "No entity with id {:?} when destroying entities",
                destroy_entity_event.server_entity_id
            );
            return;
        };

        let message = DestroyEntity::new(
            destroy_entity_event.server_entity_id,
            destroy_entity_event.destroyment_method,
        );
        send_message_to_game(
            &mut server,
            &*game_user_lookup,
            &destroy_entity_event.game_id,
            Channels::ServerCommand,
            &message,
        );

        commands.entity(*entity).despawn();
        game.entities.remove(&destroy_entity_event.server_entity_id);
    }
}
