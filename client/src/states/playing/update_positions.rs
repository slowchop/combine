use crate::net::UpdatePositionEvent;
use bevy::prelude::*;
use shared::game::position::{Position, Velocity};
use shared::game::shared_game::SharedGame;

pub fn update_positions_from_server(
    mut commands: Commands,
    mut update_positions_events: EventReader<UpdatePositionEvent>,
    game: Query<&SharedGame>,
    mut query: Query<(&mut Position, &mut Velocity)>,
) {
    let game = if let Ok(game) = game.get_single() {
        game
    } else {
        warn!("Could not get game for update_position_from_server");
        return;
    };

    for update_position_event in update_positions_events.iter() {
        let entity =
            if let Some(entity) = game.entities.get(&update_position_event.server_entity_id) {
                entity
            } else {
                warn!(
                    "Could not get entity for server_entity_id {:?}",
                    update_position_event
                );
                continue;
            };

        // commands
        //     .entity(*entity)
        //     .insert(Position(update_position_event.position))
        //     .insert(Velocity(update_position_event.velocity));

        if let Ok((mut position, mut velocity)) = query.get_mut(*entity) {
            position.0 = update_position_event.position;
            velocity.0 = update_position_event.velocity;
        } else {
            warn!(
                "Could not update position and velocity for entity {:?}",
                entity
            );
        }
    }
}

pub fn update_transforms_from_positions(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation = position.0;
    }
}

pub fn update_transform_from_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Position, &Velocity)>,
) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds();
    }
}
