use crate::game::player_name::PlayerName;

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
