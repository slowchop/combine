use crate::game_info::Owner;
use crate::player_name::PlayerName;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GameReady {
    pub player_names: Property<[String; 2]>,
    /// Sent to a player to tell them which player ID they are.
    pub you_are: Property<u8>,
    pub level: Property<String>,
}

impl GameReady {
    pub fn new(player_names: [PlayerName; 2], you_are: u8, level: String) -> Self {
        let player_names = player_names
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        GameReady::new_complete(player_names, you_are, level)
    }
}
