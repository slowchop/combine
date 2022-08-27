use crate::game::defs::{DamageType, NetVec2, NetVec3};
use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct HurtEntity {
    pub src_server_entity_id: Property<Option<ServerEntityId>>,
    pub dst_server_entity_id: Property<ServerEntityId>,
    pub total_damaged: Property<u32>,
}

impl HurtEntity {
    pub fn new(src: Option<ServerEntityId>, dst: ServerEntityId, total_damaged: u32) -> Self {
        HurtEntity::new_complete(src, dst, total_damaged)
    }
}
