use crate::game::player_name::PlayerName;

#[derive(Clone, Debug)]
pub struct Player {
    pub name: PlayerName,
}

impl Player {
    pub fn new(name: PlayerName) -> Self {
        Player { name }
    }
}
