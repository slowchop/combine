use crate::game::owner::Owner;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GameOver {
    pub winner: Property<Owner>,
}

impl GameOver {
    pub fn new(owner: Owner) -> Self {
        GameOver::new_complete(owner)
    }
}
