pub mod bottom_quad;
pub mod camera;
pub mod left_click;
pub mod spawn_entities;

use bevy::prelude::*;
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::protocol::game_ready::GameReady;

#[derive(Component)]
pub struct GameInfo {
    pub level: String,
    pub players: [SharedPlayer; 2],
    pub i_am: Owner,
}

impl From<&GameReady> for GameInfo {
    fn from(g: &GameReady) -> Self {
        let players = g
            .player_names
            .iter()
            .map(|p| SharedPlayer::new(PlayerName::new(p)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        GameInfo {
            players,
            level: (*g.map).clone(),
            i_am: Owner::new(*g.i_am),
        }
    }
}
