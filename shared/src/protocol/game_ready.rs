use crate::game_info::GameInfo;
use crate::player_name::PlayerName;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct GameReady {
    pub players: Property<[(u8, u8); 2]>,
    /// Sent to a player to tell them which player ID they are.
    pub you_are: Property<u8>,
    pub level: Property<String>,
}

impl GameReady {
    pub fn new(players: [PlayerName; 2], you_are: u8, level: String) -> Self {
        let players: [(u8, u8); 2] = players
            .iter()
            .map(|v| v.to_tuple())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        GameReady::new_complete(players, you_are, level)
    }
}

impl From<&GameReady> for GameInfo {
    fn from(g: &GameReady) -> Self {
        GameInfo {
            players: (*g.players)
                .iter()
                .map(|v| PlayerName::from(*v))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            level: (*g.level).clone(),
            you_are: (*g.you_are) as usize,
        }
    }
}
