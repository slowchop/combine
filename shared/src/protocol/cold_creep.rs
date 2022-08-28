use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ColdCreep {
    pub server_entity_id: Property<ServerEntityId>,
    pub duration: Property<f32>,
}

impl ColdCreep {
    pub fn new(creep: ServerEntityId, duration: f32) -> Self {
        Self::new_complete(creep, duration)
    }
}
