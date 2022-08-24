use crate::game::defs::{NetVec2, NetVec3};
use crate::game::shared_game::ServerEntityId;
use crate::Ticks;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct UpdatePosition {
    pub server_entity_id: Property<ServerEntityId>,
    pub position: Property<NetVec3>,
    pub velocity: Property<NetVec3>,
}

impl UpdatePosition {
    pub fn new(server_entity_id: ServerEntityId, position: Vec3, velocity: Vec3) -> Self {
        UpdatePosition::new_complete(server_entity_id, position.into(), velocity.into())
    }
}
