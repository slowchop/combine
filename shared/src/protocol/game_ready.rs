use crate::game::owner::Owner;
use crate::game::player::{PlayerName, SharedPlayer};
use crate::game::ClientGameInfo;
use crate::seen_hack;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GameReady {
    pub game_info: Property<ClientGameInfo>,
}

impl GameReady {
    pub fn new(game_info: ClientGameInfo) -> Self {
        GameReady::new_complete(game_info)
    }
}
