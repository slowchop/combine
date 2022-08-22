use crate::game::defs::EntityDef;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct SpawnEntity {
    pub entity_def: Property<EntityDef>,
}

impl SpawnEntity {
    pub fn new(e: &EntityDef) -> Self {
        SpawnEntity::new_complete(e.clone())
    }
}
