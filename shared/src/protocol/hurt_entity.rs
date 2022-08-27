use crate::game::defs::{NetVec2, NetVec3};
use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct HurtEntity {
    pub server_entity_id: Property<ServerEntityId>,
    pub total_damage: Property<u32>,
}

impl HurtEntity {
    pub fn new(server_entity_id: ServerEntityId, total_damage: u32) -> Self {
        HurtEntity::new_complete(server_entity_id, total_damage)
    }
}
