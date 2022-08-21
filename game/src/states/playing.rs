pub mod bottom_quad;
pub mod camera;
pub mod left_click;
pub mod level;
pub mod spawn_entities;

use bevy::prelude::*;
use shared::game::game_info::Owner;
use shared::game::player::{Controller, Player};
use shared::game::player_name::PlayerName;
use shared::protocol::game_ready::GameReady;

#[derive(Component)]
pub struct GameInfo {
    pub level: String,
    pub players: [Player; 2],
    pub you_are: Owner,
    pub multiplayer: bool,
}

impl From<&GameReady> for GameInfo {
    fn from(g: &GameReady) -> Self {
        let players = g
            .player_names
            .iter()
            .map(|p| Player {
                name: PlayerName::new(p),
                controller: Controller::Human,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        GameInfo {
            players,
            level: (*g.level).clone(),
            you_are: Owner::new(*g.you_are),
            multiplayer: true,
        }
    }
}
