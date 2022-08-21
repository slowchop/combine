pub mod bottom_quad;
pub mod camera;
pub mod left_click;
pub mod level;

use bevy::prelude::*;
use shared::game_info::Owner;
use shared::player_name::PlayerName;
use shared::protocol::game_ready::GameReady;

#[derive(Component)]
pub struct GameInfo {
    pub level: String,
    pub players: [Player; 2],
    pub you_are: Owner,
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
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: PlayerName,
    pub controller: Controller,
}

impl Player {
    pub fn human(name: PlayerName) -> Self {
        Player {
            name,
            controller: Controller::Human,
        }
    }

    pub fn ai() -> Self {
        Player {
            name: PlayerName::new("BOT"),
            controller: Controller::Ai,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Controller {
    Human,
    Ai,
}
