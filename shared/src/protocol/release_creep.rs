use crate::game::defs::{NetVec2, NetVec3};
use crate::game::shared_game::ServerEntityId;
use crate::Ticks;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ReleaseCreeps {
    pub creeps: Property<Vec<ServerEntityId>>,
}

impl ReleaseCreeps {
    pub fn new(ids: Vec<ServerEntityId>) -> Self {
        ReleaseCreeps::new_complete(ids)
    }
}
