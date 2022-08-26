use crate::states::editor::menu::NewEvent;
use crate::{Commands, Entity, EventReader, Query, With};
use shared::game::defs::EntityDef;

pub fn new_events(
    mut commands: Commands,
    mut new_events: EventReader<NewEvent>,
    query: Query<Entity, With<EntityDef>>,
) {
    for _ in new_events.iter() {
        for entity in query.iter() {
            println!("Deleting entity {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}
