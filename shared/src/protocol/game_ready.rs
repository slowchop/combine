use crate::game::game_info::Owner;
use crate::game::player_name::PlayerName;
use crate::seen_hack;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GameReady {
    pub seen: Property<u64>,
    pub player_names: Property<Vec<String>>,
    /// Sent to a player to tell them which player ID they are.
    pub i_am: Property<u8>,
    pub level: Property<String>,
}

impl GameReady {
    pub fn new(player_names: Vec<PlayerName>, i_am: u8, level: String) -> Self {
        let player_names = player_names
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>();
        GameReady::new_complete(seen_hack(), player_names, i_am, level)
    }
}
