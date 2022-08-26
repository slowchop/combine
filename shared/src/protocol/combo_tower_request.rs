use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ComboTowerRequest {
    pub towers: Property<Vec<ServerEntityId>>,
}

impl ComboTowerRequest {
    pub fn new(towers: Vec<ServerEntityId>) -> Self {
        ComboTowerRequest::new_complete(towers)
    }
}
