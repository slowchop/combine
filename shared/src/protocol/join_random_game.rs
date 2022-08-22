use crate::game::player::PlayerName;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct JoinRandomGame {
    pub name: Property<String>,
}

impl JoinRandomGame {
    pub fn new(name: PlayerName) -> Self {
        let name = name.to_string();
        JoinRandomGame::new_complete(name)
    }
}
