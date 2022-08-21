use crate::player_name::PlayerName;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct JoinRandomGame {
    pub name: Property<(u8, u8)>,
}

impl JoinRandomGame {
    pub fn new(name: PlayerName) -> Self {
        let name = name.to_tuple();
        JoinRandomGame::new_complete(name)
    }
}
