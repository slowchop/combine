use crate::net::DestroyEntityEvent;
use bevy::prelude::*;
use shared::game::shared_game::SharedGame;

pub fn destroy_entities(
    mut commands: Commands,
    mut event_reader: EventReader<DestroyEntityEvent>,
    mut game: Query<&mut SharedGame>,
) {
    for event in event_reader.iter() {
        let mut game = if let Ok(game) = game.get_single_mut() {
            game
        } else {
            warn!("No game when destroying entities");
            return;
        };

        let entity = if let Some(e) = game.entities.get(&event.server_entity_id) {
            e
        } else {
            warn!(
                "No entity with id {:?} when destroying entities",
                event.server_entity_id
            );
            return;
        };

        commands.entity(*entity).despawn();

        game.entities.remove(&event.server_entity_id);
    }
}
