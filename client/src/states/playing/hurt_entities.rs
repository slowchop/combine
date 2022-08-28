use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use rand::{thread_rng, Rng};
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
    mut lines: ResMut<DebugLines>,
    transforms: Query<&Transform>,
) {
    let game = if let Ok(game) = game.get_single() {
        game
    } else {
        warn!("No shared game in hurt entities");
        return;
    };

    for hurt_entity_event in hurt_entity_events.iter() {
        let dst_entity = if let Some(dst_entity) = game.entities.get(&hurt_entity_event.dst) {
            dst_entity
        } else {
            warn!(
                "No dst entity found for HurtEntityEvent: {:?}",
                hurt_entity_event
            );
            continue;
        };

        let dst_transform = if let Ok(d) = transforms.get(*dst_entity) {
            d
        } else {
            warn!(
                "No dst transform found for HurtEntityEvent: {:?}",
                hurt_entity_event
            );
            continue;
        };

        if let Some(src_entity) = hurt_entity_event.src {
            if let Some(src_entity) = game.entities.get(&src_entity) {
                if let Ok(src_transform) = transforms.get(*src_entity) {
                    lines.line_colored(
                        src_transform.translation + Vec3::Y * 1.0,
                        dst_transform.translation
                            + Vec3::new(
                                thread_rng().gen_range(-1.0..1.0),
                                thread_rng().gen_range(0.0..2.0),
                                0.0,
                            ),
                        0.3,
                        Color::BLUE,
                    );
                }
            }
        }

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
