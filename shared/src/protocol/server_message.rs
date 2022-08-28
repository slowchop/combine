use crate::game::owner::Owner;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ServerMessage {
    pub text: Property<String>,
}

impl ServerMessage {
    pub fn new(s: String) -> Self {
        Self::new_complete(s)
    }
}
