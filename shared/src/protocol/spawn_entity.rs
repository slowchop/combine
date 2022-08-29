use crate::game::defs::EntityDef;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct SpawnEntity {
    pub entity_def: Property<EntityDef>,
    pub health_multiplier: Property<f32>,
    pub speed_multiplier: Property<f32>,
}

impl SpawnEntity {
    pub fn new(e: &EntityDef, health_multiplier: f32, speed_multiplier: f32) -> Self {
        SpawnEntity::new_complete(e.clone(), health_multiplier, speed_multiplier)
    }
}
