use crate::game::defs::{NetVec2, NetVec3};
use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct HurtEntity {
    pub src_server_entity_id: Property<ServerEntityId>,
    pub dst_server_entity_id: Property<ServerEntityId>,
}

impl HurtEntity {
    pub fn new(src: ServerEntityId, dst: ServerEntityId) -> Self {
        HurtEntity::new_complete(src, dst)
    }
}
