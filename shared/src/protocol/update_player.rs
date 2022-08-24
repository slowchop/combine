use crate::game::owner::Owner;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct UpdatePlayer {
    pub owner: Property<Owner>,
    pub gold: Property<u32>,
    pub lives: Property<u32>,
}

impl UpdatePlayer {
    pub fn new(owner: Owner, gold: u32, lives: u32) -> Self {
        UpdatePlayer::new_complete(owner, gold, lives)
    }
}
