use crate::net::DestroyEntityEvent;
use bevy::prelude::*;
use shared::game::shared_game::SharedGame;

pub fn destroy_entities(
    mut commands: Commands,
    mut event_reader: EventReader<DestroyEntityEvent>,
    game: Query<&SharedGame>,
) {
    for event in event_reader.iter() {
        let game = if let Ok(game) = game.get_single() {
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

        info!("Destroying entity {:?}", entity);
        commands.entity(*entity).despawn();
    }
}
