use crate::game::owner::Owner;
use crate::game::player::PlayerName;
use crate::seen_hack;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GameReady {
    pub player_names: Property<Vec<String>>,
    /// Sent to a player to tell them which player ID they are.
    pub i_am: Property<u8>,
    pub map: Property<String>,
}

impl GameReady {
    pub fn new(player_names: Vec<PlayerName>, i_am: u8, map: String) -> Self {
        let player_names = player_names
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>();
        GameReady::new_complete(player_names, i_am, map)
    }

    pub fn player_names(&self) -> Vec<PlayerName> {
        self.player_names
            .as_slice()
            .iter()
            .map(|n| PlayerName::new(n.as_str()))
            .collect::<Vec<_>>()
    }
}
