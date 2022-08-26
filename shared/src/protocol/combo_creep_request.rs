use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ComboCreepRequest {
    pub creeps: Property<Vec<ServerEntityId>>,
}

impl ComboCreepRequest {
    pub fn new(creeps: Vec<ServerEntityId>) -> Self {
        ComboCreepRequest::new_complete(creeps)
    }
}
