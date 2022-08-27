use bevy::prelude::*;
use shared::game::shared_game::{ServerEntityId, SharedGame};

#[derive(Component)]
pub struct Damaged(pub u32);

#[derive(Debug)]
pub struct HurtEntityEvent {
    pub src: Option<ServerEntityId>,
    pub dst: ServerEntityId,
    pub total_damaged: u32,
}

pub fn hurt_entities(
    mut hurt_entity_events: EventReader<HurtEntityEvent>,
    mut commands: Commands,
    game: Query<&SharedGame>,
    mut query: Query<&mut Damaged>,
) {
    let game = if let Ok(game) = game.get_single() {
        game
    } else {
        warn!("No shared game in hurt entities");
        return;
    };

    for hurt_entity_event in hurt_entity_events.iter() {
        println!("HurtEntityEvent: {:?}", hurt_entity_event);
        // Find the dst entity, attach a Damaged component to it.
        let dst_entity = if let Some(dst_entity) = game.entities.get(&hurt_entity_event.dst) {
            dst_entity
        } else {
            warn!(
                "No dst entity found for HurtEntityEvent: {:?}",
                hurt_entity_event
            );
            continue;
        };

        let mut damaged = if let Ok(damaged) = query.get_mut(*dst_entity) {
            damaged
        } else {
            warn!(
                "No damaged component found for HurtEntityEvent: {:?}",
                hurt_entity_event
            );
            continue;
        };

        damaged.0 = hurt_entity_event.total_damaged;
    }
}
